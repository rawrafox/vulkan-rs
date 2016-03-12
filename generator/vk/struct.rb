module Vulkan
  StructMember = ::Struct.new("StructMember", :name, :type)

  class Struct < Vulkan::Object
    attr_accessor :guard

    def initialize(context, node)
      super(context, node, Name.new(node['name'], :struct))
    end

    def reify!
      @comment = node['comment']
      @members = node.xpath('./member').map do |member|
        name = member.xpath('./name').inner_text.underscore
        type = self.type_registry.string_lookup(member.at_xpath('./type').inner_text)
        length = member['len']
        optional = member['optional'] == 'true'

        if array_match = name.match(/\[(\w+)\]$/)
          name = name.split('[')[0]

          type = Primitive.new(self.context, nil, type, array: true, length: Name.new(array_match[1], :constant))
        elsif enum = member.at_xpath('./enum')
          type = Primitive.new(self.context, nil, type, array: true, length: Name.new(enum.inner_text, :constant))
        elsif name.match(/^p_/)
          name = name.split('p_')[1]

          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
        elsif name.match(/^pp_/)
          name = name.split('pp_')[1]

          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
        elsif name.match(/^pfn_/)
          name = name.split('pfn_')[1]

          type = Primitive.new(self.context, nil, type, pointer: true, length: length)
        end

        name = { 'type' => 'ty', 's_type' => 'structure_ty' }.fetch(name) { name }

        StructMember.new(name, type)
      end
    end

    def prelude(name, mutable: false)
      "#{name}.raw"
    end

    def attribute_prelude(name, mutable: false)
      "#{name}"
    end

    def rust_type
      "#{self.name}"
    end

    def ffi_type(namespace = '')
      "#{namespace}#{self.name}"
    end

    def generate_ffi_type(o)
      o.puts '#[repr(C)]'
      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.block "pub struct #{self.name}", comment: @comment do
        @members.each do |member|
          o.puts "pub #{member.name}: #{member.type.ffi_type},"
        end
      end
      o.puts
    end

    def generate_type(o)
      o.puts
      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.block "pub struct #{self.name}", comment: @comment do
        o.puts "pub raw: ffi::#{self.name}"
      end
      o.puts
      o.puts "#[cfg(#{self.guard})]" if self.guard
      o.block "impl #{self.name}" do
        o.block "pub unsafe fn from_ffi(raw: ffi::#{self.name}) -> #{self.name}" do
          o.puts "return #{self.name} { raw: raw };"
        end

        @members.each do |member|
          ref_hack = member.type.is_a? Vulkan::Struct
          o.puts
          o.block "pub fn #{member.name}(&self) -> #{'&' if ref_hack}#{member.type.attribute_type}" do
            if prelude = member.type.attribute_prelude("self.raw.#{member.name}")
              o.puts "return #{'&' if ref_hack}#{prelude};"
            else
              o.puts "return #{'&' if ref_hack}self.raw.#{member.name} as #{member.type.ffi_type('ffi::')};"
            end
          end
        end
      end
    end
  end
end
