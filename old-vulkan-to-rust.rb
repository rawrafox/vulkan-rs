require 'pp'

require 'nokogiri'
require 'active_support'
require 'active_support/core_ext'

Vulkan = Nokogiri::XML(File.read(ARGV[0]))

class Outputter
  attr_accessor :indent

  def initialize(stream)
    @stream = stream
    @n_indent = 0
    @indent = "\t"
  end

  def puts(line = nil)
    if line
      @stream.puts(@indent * @n_indent + line)
    else
      @stream.puts
    end
  end

  def indent
    @n_indent += 1
    yield self
    @n_indent -= 1
  end

  def block(line, &block)
    self.puts(line + ' {')
    self.indent(&block)
    self.puts("}\n\n")
  end
end

TypeMap = {
  'void' => 'libc::c_void',
  'char' => 'u8',
  'size_t' => 'usize',
  'int8_t' => 'i8',
  'int16_t' => 'i16',
  'int32_t' => 'i32',
  'int64_t' => 'i64',
  'uint8_t' => 'u8',
  'uint16_t' => 'u16',
  'uint32_t' => 'u32',
  'uint64_t' => 'u64',
  'float' => 'f32',
  'double' => 'f64',

  # These are just filled in for now, will fix when we add WSI support
  'HWND' => '*mut libc::c_void',
  'HINSTANCE' => '*mut libc::c_void',
  'ANativeWindow' => '*mut libc::c_void',
  'MirConnection' => '*mut libc::c_void',
  'MirSurface' => '*mut libc::c_void',
  'wl_display' => '*mut libc::c_void',
  'wl_surface' => '*mut libc::c_void',
  'Display' => '*mut libc::c_void',
  'Window' => '*mut libc::c_void',
  'VisualID' => '*mut libc::c_void',
  'xcb_connection_t' => '*mut libc::c_void',
  'xcb_window_t' => '*mut libc::c_void',
  'xcb_visualid_t' => '*mut libc::c_void'
}

def type_for(name, full_text)
  pointer_type = full_text.match(/\*/) != nil
  const_pointer = full_text.match(/^const/) != nil

  type = TypeMap.fetch(name) { strip_name(name) }

  array_match = name.match(/\[([0-9]+)\]$/)
  name = name.sub(/\[([0-9]+)\]$/, '')

  if array_match
    type = "[#{type}; #{array_match[1].to_i}]"
  end

  if pointer_type
    "*#{const_pointer ? 'const' : 'mut'} #{type}"
  else
    type
  end
end

def fixup_identifier(identifier)
  {
    '1d' => 'OneD',
    '2d' => 'TwoD',
    '3d' => 'ThreeD',
    '1Bit' => 'Bits1',
    '2Bit' => 'Bits2',
    '4Bit' => 'Bits4',
    '8Bit' => 'Bits8',
    '16Bit' => 'Bits16',
    '32Bit' => 'Bits32',
    '64Bit' => 'Bits64',
    'type' => 'ty'
  }.reduce(identifier) do |ident, (k, v)|
    ident.sub(/^#{k}/, v)
  end
end

def strip_name(name)
  if name[0 .. 1] == 'Vk'
    name[2 .. -1]
  elsif name[0 .. 2] == 'VK_'
    name[3 .. -1]
  else
    name
  end
end

def strip_enum_name(enum, name)
  prefix = strip_name(enum).sub(/(KHR)|(EXT)/, '')
  fixup_identifier strip_name(name).downcase.classify.remove(prefix)
end

def strip_bitmask_name(_, name)
  fixup_identifier strip_name(name)
end

def convert_value(value)
  case value
  when /f/
    value + '32'
  when /\(~/
    'usize::max_value()'
  when /ULL/
    value.gsub('ULL', '')
  when /U/
    value.gsub('U', '')
  else
    value
  end
end

def type_of_value(value)
  case value
  when /f/
    'f32'
  else
    'usize'
  end
end

Output = Outputter.new($stdout)

Output.puts '#[macro_use]'
Output.puts
Output.puts 'use libc;'

Vulkan.xpath('/registry/types/type[@category = "basetype"]').map do |x|
  Output.puts "pub type #{strip_name x.at_xpath('./name').inner_text} = #{TypeMap.fetch(x.at_xpath('./type').inner_text)};"
end

Output.puts

Vulkan.xpath('/registry/enums').map do |enum|
  case enum['type']
  when 'enum'
    Output.block "#[repr(C)] #[derive(PartialEq)] pub enum #{strip_name enum['name']}" do
      enum.xpath('enum').map do |x|
        Output.puts "#{strip_enum_name enum['name'], x['name']} = #{x['value']},"
      end
    end
  when 'bitmask'
    Output.block "bitflags!" do
      Output.block "#[repr(C)] flags #{strip_name enum['name']}: Flags" do
        enum.xpath('./enum').map do |x|
          Output.puts "const #{strip_bitmask_name enum['name'], x['name']} = #{1 << x['bitpos'].to_i},"
        end
      end
    end
  else
    enum.xpath('./enum').map do |x|
      value = x['value']
      Output.puts "pub static #{strip_name x['name']}: #{type_of_value value} = #{convert_value value};"
    end
    Output.puts
  end
end

Vulkan.xpath('/registry/types/type').map do |type|
  case type['category']
  when nil, 'enum', 'include', 'define', 'basetype'
  when 'bitmask'
    Output.puts "pub type #{strip_name type.at_xpath('./name').inner_text} = Flags; // Reserved"
  when 'handle'
    case type.at_xpath('./type').inner_text
    when 'VK_DEFINE_HANDLE'
      Output.puts "#[repr(C)] pub struct #{strip_name type.at_xpath('./name').inner_text}(pub *mut libc::c_void);"
    when 'VK_DEFINE_NON_DISPATCHABLE_HANDLE'
      Output.puts "#[repr(C)] pub struct #{strip_name type.at_xpath('./name').inner_text}(pub u64);"
    else
      raise ArgumentError, type.inspect
    end
  when 'union'
    Output.puts "pub type #{strip_name type['name']} = *mut libc::c_void; // FOOTLONG: Wildly wrong"
  when 'funcpointer'
    Output.puts "pub type #{strip_name type.at_xpath('./name').inner_text} = *mut libc::c_void; // FOOTLONG: Mildly wrong"
  when 'struct'
    Output.block "#[repr(C)] pub struct #{strip_name type['name']}" do
      type.xpath('./member').map do |x|
        name = strip_name(x.at_xpath('./name').inner_text).underscore.sub(/\[[0-9]+\]$/, '')

        Output.puts "pub #{fixup_identifier name}: #{type_for(x.at_xpath('./type').inner_text, x.inner_text)},"
      end
    end
  else
    $stderr.puts "WARN: Skipping type '#{type['category']}'"
  end
end

Output.block '#[link(name = "vulkan-1")] extern "C"' do
  Vulkan.xpath('/registry/commands/command').map do |command|
    proto = command.at_xpath('./proto')

    return_type = if proto.at_xpath('./type').inner_text != 'void'
      " -> #{strip_name proto.at_xpath('./type').inner_text}"
    end

    params = command.xpath('./param').map do |x|
      name = strip_name(x.at_xpath('./name').inner_text).underscore.sub(/\[[0-9]+\]$/, '')

      "#{fixup_identifier name}: #{type_for(x.at_xpath('./type').inner_text, x.inner_text)}"
    end

    Output.puts "pub fn #{proto.at_xpath('./name').inner_text}(#{params.join(', ')})#{return_type};"
  end
end
