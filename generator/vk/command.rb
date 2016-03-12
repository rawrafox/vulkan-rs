module Vulkan
  CommandParameter = ::Struct.new("CommandParameter", :name, :type, :optional)

  class Command < Vulkan::Object
    attr_accessor :guard, :params, :return_type

    def initialize(context, node)
      super(context, node, Name.new(node.at_xpath('./proto/name').inner_text, :command))
    end

    def reify!
      proto = self.node.at_xpath('./proto')
      return_type_name = proto.at_xpath('./type').inner_text

      @return_type = self.type_registry.string_lookup(return_type_name) if return_type_name != 'void'

      @params = self.node.xpath('./param').map do |param|
        name = param.xpath('./name').inner_text.underscore
        type = self.type_registry.string_lookup(param.at_xpath('./type').inner_text)
        length = param['len']
        optional = param['optional']

        if array_match = name.match(/\[(\w+)\]$/)
          name = name.split('[')[0]

          type = Primitive.new(self.context, nil, type, array: true, length: Name.new(array_match[1], :constant))
        elsif enum = param.at_xpath('./enum')
          type = Primitive.new(self.context, nil, type, array: true, length: Name.new(enum.inner_text, :constant))
        elsif name.match(/^p_/)
          name = name.split('p_')[1]

          type = Primitive.new(self.context, nil, type, pointer: true, length: length, const: param.inner_text =~ /const/)
        elsif name.match(/^pp_/)
          name = name.split('pp_')[1]

          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
        elsif name.match(/^pfn_/)
          name = name.split('pfn_')[1]

          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
        end

        name = { 'type' => 'ty', 's_type' => 'structure_ty' }.fetch(name) { name }

        CommandParameter.new(name, type, optional)
      end
    end

    def generate_ffi_extern_functions(o)
      return_string = " -> #{@return_type.ffi_type}" if @return_type
      parameters = @params.map { |param| "#{param.name}: #{param.type.ffi_type}"}

      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.puts "fn #{self.name.name}(#{parameters.join(', ')})#{return_string};"
    end

    def generate_ffi_functions(o)
      return_string = " -> #{@return_type.ffi_type}" if @return_type
      parameters = @params.map { |param| "#{param.name}: #{param.type.ffi_type}"}
      arguments = @params.map { |param| "#{param.name}" }

      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.block "pub unsafe fn #{self.name}(#{parameters.join(', ')})#{return_string}" do
        o.puts "return #{self.name.name}(#{arguments.join(', ')});"
      end
      o.puts
    end

    def generate_functions(o)
      return if @params[0].type.is_a? Vulkan::Handle

      raise ArgumentError, "needs to return VkResult" unless @return_type == self.context.type_registry.string_lookup('VkResult')

      count_mode = false

      preludes = @params[0 .. -2].map do |param|
        if param.name =~ /_count$/
          count_mode = true
          "let #{param.name}_prelude = &mut 0;"
        else
          if param.optional
            if prelude = param.type.prelude("x")
              "let #{param.name}_prelude = #{param.name}.map_or(std::ptr::null(), |x| { #{prelude} });"
            end
          else
            if prelude = param.type.prelude("#{param.name}")
              "let #{param.name}_prelude = #{prelude};"
            end
          end
        end
      end

      parameters = @params[0 .. -2].map do |param|
        unless param.name =~ /_count$/
          if param.optional
            "#{param.name}: Option<#{param.type.rust_type}>"
          else
            "#{param.name}: #{param.type.rust_type}"
          end
        end
      end

      arguments = @params[0 .. -2].zip(preludes).map do |param, prelude|
        if prelude
          if param.type.is_a?(Vulkan::Primitive) && param.type.pointee == self.context.type_registry.string_lookup('char')
            "#{param.name}_prelude.as_ptr()"
          else
            "#{param.name}_prelude"
          end
        else
          "#{param.name} as #{param.type.ffi_type('ffi::')}"
        end
      end

      return_string = if count_mode
        " -> std::result::Result<Vec<#{@params[-1].type.pointee.rust_type}>, ffi::Result>"
      else
        " -> std::result::Result<#{@params[-1].type.pointee.rust_type}, ffi::Result>"
      end

      o.puts
      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.block "pub fn #{self.name}(#{parameters.compact.join(', ')})#{return_string}" do
        o.block 'unsafe' do
          preludes.compact.each do |prelude|
            o.puts prelude
          end

          if count_mode
            o.puts "let result = ffi::#{self.name}(#{(arguments + ['std::ptr::null_mut()']).join(', ')});"
            o.puts "if result != ffi::Result::Success { return Err(result) };"
            arguments.push("#{@params[-1].name}.as_mut_ptr()")
            o.puts "let mut #{@params[-1].name}: Vec<#{@params[-1].type.pointee.ffi_type('ffi::')}> = Vec::with_capacity(*#{@params[-2].name}_prelude as usize);"
            o.puts "let result = ffi::#{self.name}(#{arguments.join(', ')});"
            o.puts "if result != ffi::Result::Success { return Err(result) };"
            o.puts "#{@params[-1].name}.set_len(*#{@params[-2].name}_prelude as usize);"
            o.puts "let result: Vec<#{@params[-1].type.pointee.rust_type}> = #{@params[-1].name}.into_iter().map(|x: #{@params[-1].type.pointee.ffi_type('ffi::')}| #{@params[-1].type.pointee.rust_type}::from_ffi(x)).collect();"
            o.puts "return Ok(result);"
          else
            arguments.push("&mut #{@params[-1].type.pointee.to_ffi(@params[-1].name)} as #{@params.last.type.ffi_type('ffi::')}")
            o.puts "let mut #{@params[-1].name}: #{@params[-1].type.pointee.rust_type} = std::mem::uninitialized();"
            o.puts "let result = ffi::#{self.name}(#{arguments.join(', ')});"
            o.puts "if result != ffi::Result::Success { return Err(result) };"
            o.puts "return Ok(#{@params[-1].name});"
          end
        end
      end
    end
  end
end
