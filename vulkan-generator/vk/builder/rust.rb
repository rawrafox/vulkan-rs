module Vulkan
  module Rust
    class Object < Vulkan::Object
    end

    class Type < Vulkan::Rust::Object
      representation :guard, :block

      unserialized :block
    end

    class Array < Vulkan::Rust::Object
      representation :length, :type

      def name
        "<array type: #{self.type.name}, length: #{self.length}>"
      end

      def extern_type
        "[#{self.type.extern_type}; #{self.length}]"
      end
    end

    class FunctionPointer < Vulkan::Rust::Object
      representation :type

      def name
        "<function-pointer type: #{self.type.name}>"
      end
    end

    class Reference < Vulkan::Rust::Object
      representation :type, :mutable

      def name
        "<reference type: #{self.type.name}, mutable: #{self.mutable}>"
      end

      def extern_type
        "*#{self.mutable ? 'mut' : 'const'} #{self.type.extern_type}"
      end
    end

    class Option < Vulkan::Rust::Object
      representation :type

      def name
        "<option type: #{self.type.name}>"
      end

      def extern_type
        self.type.extern_type
      end
    end

    class Slice < Vulkan::Rust::Object
      representation :length, :type, :mutable

      def name
        "<slice type: #{self.type.name}, length: #{self.length}, mutable: #{self.mutable}>"
      end

      def extern_type
        "*#{self.mutable ? 'mut' : 'const'} #{self.type.extern_type}"
      end
    end

    class String < Vulkan::Rust::Object
      representation :length

      def name
        "<string length: #{self.length}>"
      end

      def extern_type
        "*const u8"
      end
    end

    class AST < Vulkan::Rust::Object
      representation :children, :blocks

      unserialized :children

      def to_extern(o)
        o.puts '#[link(name = "vulkan-1")]'
        o.block 'extern "system"' do
          self.blocks.each { |b| b.to_extern(o) }
        end
      end

      def to_rust_types(o)
        self.blocks.each { |b| b.to_rust_types(o) }
      end
    end

    class Alias < Vulkan::Rust::Type
      representation :name, :comment, :type

      serialize :type, -> (x) { x.name }

      def extern_type
        self.name
      end

      def to_rust_type(o)
        o.puts "type #{self.name} = #{self.type.extern_type};"
      end
    end

    class BitMask < Vulkan::Rust::Type
      representation :name, :comment, :type, :values

      serialize :type, -> (x) { x.name }

      def extern_type
        self.name
      end
    end

    class BitMaskValue < Vulkan::Rust::Object
      representation :guard, :name, :value
    end

    class Block < Vulkan::Rust::Object
      representation :guard, :comment, :children

      def to_extern(o)
        self.children.each { |c| c.to_extern(o) if c.respond_to? :to_extern }
      end

      def to_rust_types(o)
        self.children.each { |c| c.to_rust_type(o) if c.respond_to? :to_rust_type }
      end
    end

    class Command < Vulkan::Rust::Type
      representation :queues, :success_codes, :error_codes, :render_pass, :command_buffer_levels, :comment, :ffi_return_type, :name, :ffi_parameters, :validity, :implicit_external_sync_parameters

      serialize :ffi_return_type, -> (x) { x.name if x }

      def to_extern(o)
        return_type = " -> #{self.ffi_return_type.extern_type}" if self.ffi_return_type

        parameters = self.ffi_parameters.map do |param|
          "#{param.name}: #{param.type.extern_type}"
        end.join(', ')

        o.puts "fn #{self.name}(#{parameters})#{return_type};"
      end
    end

    class Constant < Vulkan::Rust::Type
      representation :name, :value
    end

    class Enum < Vulkan::Rust::Type
      representation :name, :comment, :values

      def extern_type
        self.name
      end

      def to_rust_type(o)
        o.puts
        o.puts '#[repr(C)]'
        o.puts '#[derive(Clone, Copy, Debug, PartialEq)]'
        o.puts "#[cfg(#{self.guard})]" if self.guard
        o.block "pub enum #{self.name}", comment: @comment do
          self.values.each do |value|
            o.puts "#[cfg(#{value.guard})]" if value.guard
            o.puts "#{value.name} = #{value.value},"
          end
        end
      end
    end

    class EnumValue < Vulkan::Rust::Object
      representation :guard, :name, :value
    end

    class Function < Vulkan::Rust::Type
      representation :name, :comment, :text, :types

      def extern_type
        self.name
      end
    end

    class Handle < Vulkan::Rust::Type
      representation :name, :comment, :parents, :dispatchable

      serialize :parents, -> (x) { x.map(&:name) }

      def extern_type
        self.name
      end
    end

    class Ignored < Vulkan::Rust::Type
      representation :name
    end

    class Member < Vulkan::Rust::Object
      representation :no_auto_validity, :type, :name

      serialize :type, -> (x) { x.name }
    end

    class Parameter < Vulkan::Rust::Object
      representation :no_auto_validity, :external_sync, :type, :name

      serialize :type, -> (x) { x.name }
    end

    class Primitive < Vulkan::Rust::Type
      representation :name, :comment

      unserialized :block

      def extern_type
        {
          'void' => 'libc::c_void',
          'char' => 'libc::c_char',
          'size_t' => 'libc::size_t',
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
        }.fetch(self.name)
      end
    end

    class Struct < Vulkan::Rust::Type
      representation :name, :comment, :members, :validity

      def extern_type
        self.name
      end
    end

    class Union < Vulkan::Rust::Type
      representation :name, :comment, :members, :validity

      def extern_type
        self.name
      end
    end

    class Builder < Vulkan::Builder
      def initialize(normalized_ast)
        @normalized_ast = normalized_ast

        raise ArgumentError, 'not a basic ast' unless @normalized_ast.is_a? Vulkan::Normalized::AST

        super()
      end

      def build_ast
        raise ArgumentError if @normalized_ast.features.length != 1

        lookup = {}

        blocks = @normalized_ast.features.flat_map do |feature|
          feature.types.each do |type|
            ty = case type
            when Vulkan::Normalized::Alias
              Alias.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, type: build_type(type.type, lookup))
            when Vulkan::Normalized::BitMask
              values = type.values.map do |value|
                BitMaskValue.new(guard: build_guard(value.guard), name: value.name, value: build_enum_value(value))
              end

              BitMask.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, type: build_type(type.type, lookup), values: values)
            when Vulkan::Normalized::Constant
              Constant.new(guard: build_guard(type.guard), block: nil, name: type.name, value: type.value)
            when Vulkan::Normalized::Define, Vulkan::Normalized::Include
              Ignored.new(guard: build_guard(type.guard), block: nil, name: type.name)
            when Vulkan::Normalized::Enum
              values = type.values.map do |value|
                EnumValue.new(guard: build_guard(value.guard), name: value.name, value: build_enum_value(value))
              end

              Enum.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, values: values)
            when Vulkan::Normalized::Function
              Function.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, text: type.text, types: type.types)
            when Vulkan::Normalized::Handle
              parents = type.parents.map { |parent| lookup.fetch(parent) }

              Handle.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, parents: parents, dispatchable: type.dispatchable)
            when Vulkan::Normalized::Primitive
              Primitive.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment)
            when Vulkan::Normalized::Struct, Vulkan::Normalized::Union
              members = type.members.map do |member|
                raise ArgumentError, "No external synchronized struct members" if member.external_sync
                Member.new(no_auto_validity: member.no_auto_validity, type: build_type(member.type, lookup), name: member.name)
              end

              { Vulkan::Normalized::Struct => Struct, Vulkan::Normalized::Union => Union }.fetch(type.class).new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, members: members, validity: type.validity)
            else
              raise ArgumentError, type.class
            end

            lookup[ty.name] = ty
          end

          feature.commands.each do |command|
            return_type = build_type(command.return_type, lookup) unless command.return_type.name == 'void'

            parameters = command.parameters.map do |parameter|
              Parameter.new(no_auto_validity: parameter.no_auto_validity, external_sync: parameter.external_sync, type: build_type(parameter.type, lookup), name: parameter.name)
            end

            cmd = Command.new(guard: build_guard(command.guard), block: nil, name: command.name, queues: command.queues, success_codes: command.success_codes, error_codes: command.error_codes, render_pass: command.render_pass, command_buffer_levels: command.command_buffer_levels, comment: command.comment, ffi_return_type: return_type, ffi_parameters: parameters, validity: command.validity, implicit_external_sync_parameters: command.implicit_external_sync_parameters)

            lookup[cmd.name] = cmd
          end

          feature.blocks.map do |block|
            b = Block.new(guard: build_guard(@normalized_ast.features.first), comment: block.comment, children: nil)

            b.children = block.children.map do |child|
              lookup.fetch(child.name).tap do |v|
                v.block = block
              end
            end

            b
          end
        end

        unassigned_block = Block.new(guard: nil, comment: "Types not assigned to a block in Vulkan", children: nil)

        unassigned_block.children = lookup.values.reject { |value| value.block }
        unassigned_block.children.each { |child| child.block = unassigned_block }

        blocks.unshift(unassigned_block)

        AST.new(children: lookup.values, blocks: blocks)
      end

      def build_enum_value(value)
        value.value || "0b#{(1  << value.bit_position.to_i).to_s(2)}"
      end

      def build_guard(guard)
        unless guard.is_a? Vulkan::Normalized::Feature
          guard.name if guard
        end
      end

      def build_type(type, rust_types)
        case type
        when Vulkan::Normalized::Pointer
          ty = if type.type.respond_to?(:name) && type.type.name == 'char'
            raise ArgumentError if type.mutable
            String.new(length: type.length)
          else
            if type.length
              Slice.new(length: type.length, type: build_type(type.type, rust_types), mutable: type.mutable)
            else
              Reference.new(type: build_type(type.type, rust_types), mutable: type.mutable)
            end
          end

          ty = Option.new(type: ty) if type.optional

          ty
        when Vulkan::Normalized::Array
          if type.type.name == 'char'
            String.new(length: type.length)
          else
            Array.new(length: type.length, type: build_type(type.type, rust_types))
          end
        when Vulkan::Normalized::FunctionPointer
          ty = FunctionPointer.new(type: build_type(type.type, rust_types))

          ty = Option.new(type: ty) if type.optional

          ty
        else
          rust_types.fetch(type.name)
        end
      end
    end
  end
end
