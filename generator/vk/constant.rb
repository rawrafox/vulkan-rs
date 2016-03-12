module Vulkan
  class Constant < Vulkan::Object
    def initialize(context, node)
      super(context, node, Name.new(node['name'], :constant))
    end

    def reify!
      value = self.node['value']

      @value = case value
      when /f/
        value + '32'
      when /\(~/
        'usize::max_value()'
      when /ULL/
        value.gsub('ULL', '')
      when /U/
        value.gsub('U', '')
      else
        value
      end

      @type = self.type_registry.string_lookup(value =~ /f/ ? 'float' : 'size_t')
    end

    def generate_ffi_type_aliases(o)
      o.puts "pub const #{self.name}: #{@type.ffi_type} = #{@value};"
    end

    def generate_type(o)
      o.puts "pub const #{self.name}: #{@type.rust_type} = ffi::#{self.name} as #{@type.rust_type};"
    end
  end
end
