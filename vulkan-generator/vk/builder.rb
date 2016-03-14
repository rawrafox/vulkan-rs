module Vulkan
  class Builder
    attr_reader :ast

    def initialize
      @ast = self.build_ast
    end
  end
end
