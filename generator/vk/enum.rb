module Vulkan
  EnumMember = ::Struct.new("EnumMember", :name, :value, :comment)

  class Enum < Vulkan::Object
    attr_accessor :guard

    def initialize(context, node)
      super(context, node, Name.new(node['name'], :enum))
    end

    def reify!
      @comment = node['comment']
      @members = node.xpath('./enum').map do |member|
        name = Name.new(member['name'], :enum_member, node['expand'])
        value = member['value'] || (1 << member['bitpos'].to_i)
        comment = member['comment']

        EnumMember.new(name, value, comment)
      end
    end

    def rust_type
      "ffi::#{self.name}"
    end

    def ffi_type(namespace = '')
      "#{namespace}#{self.name}"
    end

    def generate_ffi_type(o)
      o.puts '#[repr(C)]'
      o.puts '#[derive(Clone, Copy, Debug, PartialEq)]'
      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.block "pub enum #{self.name}", comment: @comment do
        needs_prefix = @members.any? { |member| member.name.to_s =~ /^[0-9]/ }

        @members.each do |member|
          o.puts "#{self.name if needs_prefix}#{member.name} = #{member.value},", comment: member.comment
        end
      end

      o.puts
    end

    def generate_type(o)
      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.puts "pub type #{self.name} = ffi::#{self.name};"
    end
  end
end
