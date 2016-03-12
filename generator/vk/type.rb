module Vulkan
  class TypeRegistry
    attr_reader :types

    def initialize(context)
      @context = context
      @types = {}
    end

    def register(type, name: nil)
      name ||= type.name

      raise ArgumentError unless name.is_a? Name

      @types[name] = type
    end

    def register_c_types
      [
        ['void', 'libc::c_void', '()'],
        ['char', 'libc::c_char', 'u8'],
        ['size_t', 'libc::size_t', 'usize'],
        ['int8_t', 'i8', 'i8'],
        ['int16_t', 'i16', 'i16'],
        ['int32_t', 'i32', 'i32'],
        ['int64_t', 'i64', 'i64'],
        ['uint8_t', 'u8', 'u8'],
        ['uint16_t', 'u16', 'u16'],
        ['uint32_t', 'u32', 'u32'],
        ['uint64_t', 'u64', 'u64'],
        ['float', 'f32', 'f32'],
        ['double', 'f64', 'f64']
      ].each { |k, v, r| self.register(Primitive.new(@context, k, v, rust_type: r)) }

      # FOOTLONG: These are just filled in for now, will fix when we add WSI support
      ['HWND', 'HINSTANCE', 'ANativeWindow', 'MirConnection', 'MirSurface', 'wl_display', 'wl_surface', 'Display', 'Window', 'VisualID', 'xcb_connection_t', 'xcb_window_t', 'xcb_visualid_t'].each do |k|
        self.register(Primitive.new(@context, k, 'libc::size_t', rust_type: 'usize'))
      end

      # FOOTLONG: These are just filled in until we have union support :C
      ['VkClearColorValue', 'VkClearValue'].each do |k|
        self.register(Primitive.new(@context, k, 'libc::size_t', rust_type: 'usize'))
      end
    end

    def lookup(name)
      @types.fetch(name)
    end

    def string_lookup(name)
      @types.fetch(Name.new(name))
    end
  end

  class Primitive < Vulkan::Object
    attr_reader :pointer, :array, :optional

    def initialize(context, name, ffi_type, array: false, pointer: false, const: true, optional: false, length: nil, rust_type: nil)
      super(context, nil, Name.new(name, :primitive))

      @array, @pointer, @const, @optional, @length = array, pointer, const, optional, length
      @ffi_type, @rust_type = ffi_type, rust_type
    end

    def pointee
      @ffi_type if @pointer
    end

    def length
      case @length
      when /(\w+),null-terminated$/
        "self.#{$~[1].underscore}()"
      when /^latexmath:\[\$(.+)\$\]$/
        code = $~[1]
        case code
        when /^(\w+) \\over (\w+)$/
          "self.#{$~[1].underscore}() / #{$~[2]}"
        when /^\\lceil{\\mathit{(\w+)} \\over (\w+)}\\rceil$/
          "(self.#{$~[1].underscore}().bits() as usize + #{$~[2]} - 1) / #{$~[2]}"
        else
          raise ArgumentError, "#{code.inspect} is unsupported latexmath"
        end
      when Vulkan::Name
        @length
      else
        "self.#{@length.underscore}()"
      end
    end

    def prelude(name)
      if @pointer
        if @ffi_type == context.type_registry.string_lookup('char')
          "std::ffi::CString::new(#{name}).unwrap()"
        else
          if @length
            "unsafe { std::slice::from_raw_parts(#{name}, #{self.length} as usize) }"
          else
            if prelude = @ffi_type.prelude(name)
              "&#{@const ? '' : 'mut '}#{prelude} as #{self.ffi_type('ffi::')}"
            end
          end
        end
      elsif @array
        if @ffi_type == context.type_registry.string_lookup('char')
          "unsafe { std::ffi::CStr::from_ptr(#{name}.as_ptr()).to_str().unwrap() }"
        else
          "&#{name}"
        end
      end
    end

    def attribute_type
      if @pointer
        if @ffi_type == context.type_registry.string_lookup('char')
          "&str"
        else
          if @length
            if @ffi_type.is_a?(Vulkan::Primitive) && @ffi_type.pointee
              "Vec<#{@ffi_type.attribute_type}>"
            else
              "&[#{@ffi_type.attribute_type}]"
            end
          else
            "&#{@ffi_type.attribute_type}"
          end
        end
      elsif @array
        if @ffi_type == context.type_registry.string_lookup('char')
          "&str"
        else
          "&[#{@ffi_type.attribute_type}; #{self.length}]"
        end
      else
        "#{@ffi_type}"
      end
    end

    def attribute_prelude(name)
      if @pointer
        if @ffi_type == context.type_registry.string_lookup('char')
          "unsafe { std::ffi::CStr::from_ptr(#{name}).to_str().unwrap() }"
        else
          if @length
            if @ffi_type.is_a?(Vulkan::Primitive) && @ffi_type.pointee
              "unsafe { let v = Vec::with_capacity(#{self.length} as usize); v }"
            else
              "unsafe { std::slice::from_raw_parts(#{name}, #{self.length} as usize) }"
            end
          else
            value = @ffi_type.attribute_prelude(name) || name

            "unsafe { &*#{value} as &#{@ffi_type.ffi_type('ffi::')} }"
          end
        end
      elsif @array
        if @ffi_type == context.type_registry.string_lookup('char')
          "unsafe { std::ffi::CStr::from_ptr(#{name}.as_ptr()).to_str().unwrap() }"
        else
          "&#{name}"
        end
      end
    end

    def rust_type
      if @pointer
        if @ffi_type == context.type_registry.string_lookup('char')
          "&str"
        else
          "&#{@const ? '' : 'mut '}#{@ffi_type.rust_type}"
        end
      elsif @array
        if @ffi_type == context.type_registry.string_lookup('char')
          "&str"
        else
          "&[#{@ffi_type.ffi_type('ffi::')}; #{@length}]"
        end
      else
        @rust_type || "#{self.ffi_type('ffi::')}"
      end
    end

    def ffi_type(namespace = '')
      if @pointer
        "*#{@const ? 'const' : 'mut'} #{@ffi_type.ffi_type(namespace)}"
      elsif @array
        "[#{@ffi_type.ffi_type(namespace)}; #{@length}]"
      else
        "#{@ffi_type}"
      end
    end
  end
end
