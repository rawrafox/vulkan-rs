module Vulkan
  BitfieldMember = ::Struct.new("BitfieldMember", :name, :value, :comment)

  class Bitfield < Vulkan::Object
    attr_accessor :guard

    def initialize(context, node)
      super(context, node, Name.new(node.at_xpath('./name').inner_text, :bitfield))

      if requires = self.node['requires']
        @members_node = self.document.at_xpath("/registry/enums[@name=#{requires.inspect}]")

        self.type_registry.register(self, name: Name.new(@members_node['name'], :bitfield))
      end
    end

    def reify!
      @comment = node['comment']
      @type = self.type_registry.string_lookup(node.at_xpath('./type').inner_text)

      if @members_node
        @members = @members_node.xpath('./enum').map do |member|
          name = Name.new(member['name'], :bitfield_member)
          value = member['value'] || (1 << member['bitpos'].to_i)
          comment = member['comment']

          BitfieldMember.new(name, value, comment)
        end
      else
        @members = []
      end
    end

    def rust_type
      "ffi::#{self.name}"
    end

    def ffi_type(namespace = '')
      "#{namespace}#{self.name}"
    end

    def generate_ffi_type_aliases(o)
      if @members.length == 0
        o.puts "pub type #{self.name} = #{@type.ffi_type};"
      end
    end

    def generate_ffi_type(o)
      if @members.length > 0
        o.block 'bitflags!' do
          o.puts '#[repr(C)]'
          o.block "flags #{self.name}: #{@type.ffi_type}", comment: @comment do
            @members.each do |member|
              o.puts "const #{member.name} = #{member.value},", comment: member.comment
            end
          end
        end

        o.puts
      end
    end

    def generate_type(o)
      o.puts "pub type #{self.name} = ffi::#{self.name};"
    end
  end
end
