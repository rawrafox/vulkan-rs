module Vulkan
  class Extension < Vulkan::Object
    def initialize(context, node)
      super(context, node, Name.new(node['name'], :extension))
    end

    def reify!
      @guard = self.node['name'].downcase

      self.node.xpath('./require').each do |n|
        # FOOTLONG: Handle enum

        n.xpath('./type').each do |command|
          self.type_registry.string_lookup(command['name']).guard = @guard
        end

        n.xpath('./command').each do |command|
          self.type_registry.string_lookup(command['name']).guard = @guard
        end
      end
    end
  end
end
