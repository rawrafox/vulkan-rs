module Vulkan
  class Object
    attr_reader :context, :node, :name

    def initialize(context, node, name)
      @context, @node, @name = context, node, name

      self.type_registry.register(self)
    end

    def document
      @context.document
    end

    def type_registry
      @context.type_registry
    end

    def reify!
    end

    def to_ffi(name)
      "#{name}"
    end

    def prelude(_)
    end

    def attribute_prelude(_)
    end

    def attribute_type
      self.ffi_type('ffi::')
    end

    def epilogue(_)
    end

    def generate_ffi_type_aliases(_)
    end

    def generate_ffi_type(_)
    end

    def generate_ffi_extern_functions(_)
    end

    def generate_ffi_functions(_)
    end

    def generate_type(_)
    end

    def generate_functions(_)
    end
  end
end
