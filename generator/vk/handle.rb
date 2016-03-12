module Vulkan
  class Handle < Vulkan::Object
    def initialize(context, node)
      super(context, node, Name.new(node.at_xpath('./name').inner_text, :handle))
    end

    def reify!
      @dispatchable = case node.at_xpath('./type').inner_text
      when 'VK_DEFINE_HANDLE' then true
      when 'VK_DEFINE_NON_DISPATCHABLE_HANDLE' then false
      else
        raise ArgumentError, "unknown handle type #{ty.inspect}"
      end
    end

    def to_ffi(name)
      "#{name}.handle"
    end

    def rust_type
      "#{self.name}"
    end

    def ffi_type(namespace = '')
      @dispatchable ? 'libc::size_t' : 'u64'
    end

    def generate_ffi_type_aliases(o)
      o.puts "pub type #{self.name} = #{self.ffi_type};"
    end

    def generate_type(o)
      o.puts
      o.block "pub struct #{self.name}", comment: @comment do
        o.puts "pub handle: ffi::#{self.name}"
      end
      o.puts
      o.block "impl #{self.name}" do
        o.block "pub fn from_ffi(raw: ffi::#{self.name}) -> #{self.name}" do
          o.puts "return #{self.name} { handle: raw };"
        end

        self.context.objects.select { |x| x.is_a?(Vulkan::Command) && x.params.first.type == self }.each do |command|
          return_string = " -> #{command.return_type.ffi_type('ffi::')}" if command.return_type
          parameters = command.params[1 .. -1].map { |param| "#{param.name}: #{param.type.ffi_type('ffi::')}"}
          arguments = command.params[1 .. -1].map { |param| "#{param.name} as #{param.type.ffi_type('ffi::')}" }

          o.puts
          o.puts "#[cfg(#{command.guard})]" if command.guard
          o.block "pub unsafe fn #{command.name}(&self, #{parameters.join(', ')})#{return_string}" do
            o.puts "return ffi::#{command.name}(self.handle, #{arguments.join(', ')});"
          end
        end
      end
    end
  end
end
=begin
    def self.from_xml(xml, context)
      xml.xpath("/registry/types/type[@category='handle']").map do |handle|
        ty = handle.at_xpath('./type').inner_text

        dispatchable = case ty
        when 'VK_DEFINE_HANDLE' then true
        when 'VK_DEFINE_NON_DISPATCHABLE_HANDLE' then false
        else
          raise ArgumentError, "unknown handle type #{ty.inspect}"
        end

        name = Name.new(handle.at_xpath('./name').inner_text, :struct)

        self.new(context, name, dispatchable)
      end
    end

    def self.to_rust_ffi(handles, o)
      handles.each do |handle|
        o.puts "type #{handle.name} = #{handle.ffi_type};"
      end
    end

    def self.to_rust(handles, o)
      handles.each do |handle|
        o.block "pub struct #{handle.name}" do
          o.puts "vulkan_handle: #{handle.ffi_type},"
          o.puts "_phantom: marker::PhantomData<*mut ()>"
        end
      end
    end

    def initialize(context, name, dispatchable)
      super(context)

      @name = name
      @dispatchable = dispatchable

      self.type_registry.register(self)
    end

    def ffi_type
      self.dispatchable ? 'libc::size_t' : 'u64'
    end
  end
end
=end
