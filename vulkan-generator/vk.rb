require 'pp'
require 'json'

require 'nokogiri'
require 'active_support'
require 'active_support/core_ext'

$: << __dir__

require 'vk/object'

require 'vk/builder'
require 'vk/builder/basic'
require 'vk/builder/normalized'
require 'vk/builder/rust'
require 'vk/generator'
require 'vk/output'

basic = Vulkan::Basic::Builder.new(Nokogiri::XML(File.read(ARGV[0])))
normalized = Vulkan::Normalized::Builder.new(basic.ast)
rust = Vulkan::Rust::Builder.new(normalized.ast)

# pp rust.ast
