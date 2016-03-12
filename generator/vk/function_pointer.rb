module Vulkan
  class FunctionPointer < Vulkan::Object
    def initialize(context, node)
      super(context, node, Name.new(node.at_xpath('./name').inner_text, :function_pointer))
    end

    def reify!
    end

    def rust_type
      "ffi::#{self.name}"
    end

    def ffi_type(namespace = '')
      "#{namespace}#{self.name}"
    end

    def generate_ffi_type_aliases(o)
      o.puts "pub type #{self.name} = libc::size_t;"
    end
  end
end
