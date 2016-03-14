module Vulkan
  module GeneratorXpath
    def name
      @xml.name
    end

    def children
      @xml.children.select { |x| x.is_a? Nokogiri::XML::Element }.map { |x| node_type.new(x) }
    end

    def inner_text
      @xml.inner_text
    end

    def at(xpath, optional: false)
      nodes = @xml.xpath(xpath)

      return if optional && nodes.length == 0

      raise ArgumentError, "wrong result at #{xpath} (#{nodes.length} nodes found)" unless nodes.length == 1

      node_type.new(nodes.first)
    end

    def each(xpath)
      @xml.xpath(xpath).each do |node|
        yield node_type.new(node)
      end
    end

    def map(xpath)
      @xml.xpath(xpath).map do |node|
        yield node_type.new(node)
      end
    end

    def string(xpath, optional: false)
      if node = at(xpath, optional: optional)
        node.inner_text
      end
    end
  end

  class GeneratorNode
    include GeneratorXpath

    def initialize(xml)
      @xml = xml
    end

    def node_type
      self.class
    end

    def [](key)
      @xml[key.to_s]
    end
  end
end
