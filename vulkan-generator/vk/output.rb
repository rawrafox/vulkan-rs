module Vulkan
  class Output
    attr_accessor :indent

    def initialize(stream)
      @stream = stream
      @n_indent = 0
      @indent = "\t"
    end

    def puts(line = nil, comment: nil)
      if line
        line = line + " // #{comment}" if comment
        @stream.puts(@indent * @n_indent + line)
      else
        @stream.puts(comment ? "#{@indent * @n_indent}// #{comment}" : '')
      end
    end

    def indent
      @n_indent += 1
      yield self
      @n_indent -= 1
    end

    def block(line, comment: nil, &block)
      self.puts(line + ' {', comment: comment)
      self.indent(&block)
      self.puts("}\n")
    end
  end
end
