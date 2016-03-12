module Vulkan
  class BaseType < Vulkan::Object
    def initialize(context, node)
      super(context, node, Name.new(node.at_xpath('./name').inner_text, :base_type))
    end

    def reify!
      @type = self.type_registry.string_lookup(node.at_xpath('./type').inner_text)
    end

    def rust_type
      "ffi::#{self.name}"
    end

    def ffi_type(namespace = '')
      "#{namespace}#{self.name}"
    end

    def generate_ffi_type_aliases(o)
      o.puts "pub type #{self.name} = #{@type.ffi_type};"
    end

    def generate_type(o)
      o.puts "pub type #{self.name} = ffi::#{self.name};"
    end
  end
end
