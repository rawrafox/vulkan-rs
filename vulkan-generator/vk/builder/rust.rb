module Vulkan
  module Rust
    class Object < Vulkan::Object
    end

    class AST < Vulkan::Rust::Object
      representation :types
    end

    class Alias < Vulkan::Rust::Object
      representation :guard, :name, :comment, :type
    end

    class BitMask < Vulkan::Rust::Object
      representation :guard, :block, :name, :comment, :type, :values
    end

    class BitMaskValue < Vulkan::Rust::Object
      representation :guard, :name, :value
    end

    class Block < Vulkan::Normalized::Object
      representation :guard, :comment
    end

    class Constant < Vulkan::Normalized::Object
      representation :guard, :block, :name, :value
    end

    class Enum < Vulkan::Rust::Object
      representation :guard, :block, :name, :comment, :values
    end

    class EnumValue < Vulkan::Rust::Object
      representation :guard, :name, :value
    end

    class Function < Vulkan::Rust::Object
      representation :guard, :name, :comment, :text, :types
    end

    class Handle < Vulkan::Rust::Object
      representation :guard, :name, :comment, :parents, :dispatchable
    end

    class Ignored < Vulkan::Rust::Object
      representation :guard, :block, :name
    end

    class Member < Vulkan::Rust::Object
      representation :no_auto_validity, :external_sync, :type, :name
    end

    class Primitive < Vulkan::Rust::Object
      representation :guard, :name, :comment
    end

    class Struct < Vulkan::Rust::Object
      representation :guard, :block, :name, :comment, :members, :validity
    end

    class Union < Vulkan::Rust::Object
      representation :guard, :block, :name, :comment, :members, :validity
    end


    class Builder < Vulkan::Builder
      def initialize(normalized_ast)
        @normalized_ast = normalized_ast

        raise ArgumentError, 'not a basic ast' unless @normalized_ast.is_a? Vulkan::Normalized::AST

        super()
      end

      def build_ast
        raise ArgumentError if @normalized_ast.features.length != 1

        rust_types = {}

        @normalized_ast.features.first.types.each do |type|
          ty = case type
          when Vulkan::Normalized::Alias
            Alias.new(guard: build_guard(type.guard), name: type.name, comment: type.comment, type: build_type(type.type, rust_types))
          when Vulkan::Normalized::BitMask
            values = type.values.map do |value|
              BitMaskValue.new(guard: build_guard(value.guard), name: value.name, value: build_enum_value(value))
            end

            BitMask.new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, type: build_type(type.type, rust_types), values: values)
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
            Function.new(guard: build_guard(type.guard), name: type.name, comment: type.comment, text: type.text, types: type.types)
          when Vulkan::Normalized::Handle
            parents = type.parents.map { |parent| rust_types.fetch(parent) }

            Handle.new(guard: build_guard(type.guard), name: type.name, comment: type.comment, parents: parents, dispatchable: type.dispatchable)
          when Vulkan::Normalized::Primitive
            Primitive.new(guard: build_guard(type.guard), name: type.name, comment: type.comment)
          when Vulkan::Normalized::Struct, Vulkan::Normalized::Union
            # TODO: Fill out members

            { Vulkan::Normalized::Struct => Struct, Vulkan::Normalized::Union => Union }.fetch(type.class).new(guard: build_guard(type.guard), block: nil, name: type.name, comment: type.comment, members: [], validity: type.validity)
          else
            raise ArgumentError, type.class
          end

          rust_types[ty.name] = ty
        end

        blocks = @normalized_ast.features.first.blocks.each do |block|
          b = Block.new(guard: build_guard(@normalized_ast.features.first), comment: block.comment)

          block.children.each do |child|
            rust_types.fetch(child.name).block = block
          end

          b
        end

        AST.new(types: rust_types.values, blocks: blocks)
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
        rust_types.fetch(type.name)
      end
    end
  end
end
