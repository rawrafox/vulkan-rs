require 'pp'

require 'nokogiri'
require 'active_support'
require 'active_support/core_ext'

$: << __dir__

require 'vk/object'

require 'vk/base_type'
require 'vk/bitfield'
require 'vk/command'
require 'vk/constant'
require 'vk/enum'
require 'vk/extension'
require 'vk/function_pointer'
require 'vk/handle'
require 'vk/name'
require 'vk/output'
require 'vk/struct'
require 'vk/type'

module Vulkan
  class Context
    attr_reader :document, :type_registry, :objects, :extensions

    def self.from_xml(xml)
      context = Context.new(xml)

      {
        "/registry/types/type[@category='basetype']" => Vulkan::BaseType,
        "/registry/types/type[@category='bitmask']" => Vulkan::Bitfield,
        "/registry/enums[@type='enum']" => Vulkan::Enum,
        "/registry/enums[@name='API Constants']/enum" => Vulkan::Constant,
        "/registry/types/type[@category='funcpointer']" => Vulkan::FunctionPointer,
        "/registry/types/type[@category='handle']" => Vulkan::Handle,
        "/registry/types/type[@category='struct']" => Vulkan::Struct,
        "/registry/commands/command" => Vulkan::Command
      }.each do |k, v|
        context.objects.push *xml.xpath(k).map { |node| v.new(context, node) }
      end

      xml.xpath("/registry/extensions/extension").each { |node| context.extensions.push Vulkan::Extension.new(context, node) }

      context.extensions.each { |o| o.reify! }
      context.objects.each { |o| o.reify! }

      context
    end

    def initialize(xml)
      @document = xml
      @objects, @extensions = [], []
      @type_registry = TypeRegistry.new(self)
      @type_registry.register_c_types
    end

    def to_rust_ffi(stream)
      output = Output.new(stream)
      output.puts 'use libc;'
      output.puts
      @objects.each { |object| object.generate_ffi_type_aliases(output) }
      output.puts
      @objects.each { |object| object.generate_ffi_type(output) }
      output.puts
      output.puts '#[link(name = "vulkan-1")]'
      output.block 'extern "C"' do
        @objects.each { |object| object.generate_ffi_extern_functions(output) }
      end
      output.puts
      @objects.each { |object| object.generate_ffi_functions(output) }
      nil
    end

    def to_rust_structures(stream)
      output = Output.new(stream)
      output.puts 'use std;'
      output.puts
      output.puts 'use libc;'
      output.puts
      output.puts 'use ffi;'
      output.puts
      @objects.each { |object| object.generate_type(output) }
      # output.puts
      # @objects.each { |object| object.generate_functions(output) }
      nil
    end
  end

  def self.from_xml(xml)
    Vulkan::Context.from_xml(xml)
  end
end

vk = Vulkan.from_xml(Nokogiri::XML(File.read(ARGV[0])))

File.open('src/ffi.rs', 'w+') { |f| vk.to_rust_ffi(f) }
File.open('src/structures.rs', 'w+') { |f| vk.to_rust_structures(f) }
