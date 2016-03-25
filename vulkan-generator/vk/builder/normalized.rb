require 'tsort'

module Vulkan
  module Normalized
    class Sorter
      include TSort

      def initialize(hash)
        @hash = hash
      end

      def tsort_each_node(&block)
        @hash.each_key(&block)
      end

      def tsort_each_child(node, &block)
        value = @hash.fetch(node)

        requires = [value.requires]

        requires.push(*value.parents)

        case value
        when Vulkan::Basic::Type
          case value.category
          when 'basetype', 'bitmask'
            requires.push(value.definition.type)
          when 'struct', 'union'
            requires.push(*value.definition.children.select { |x| x.is_a? Vulkan::Basic::Member }.map(&:type))
          end
        end

        requires.compact!

        requires.each(&block)
      end
    end

    class Object < Vulkan::Object
    end

    class AST < Vulkan::Normalized::Object
      representation :features, :extensions, :metadata
    end

    class Alias < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :type
    end

    class Array < Vulkan::Normalized::Object
      representation :length, :type
    end

    class BitMask < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :type, :values
    end

    class BitMaskValue < Vulkan::Normalized::Object
      representation :guard, :name, :value, :bit_position, :api, :type, :name_alias
    end

    class Block < Vulkan::Normalized::Object
      representation :comment, :children
    end

    class Command < Vulkan::Normalized::Object
      representation :guard, :block, :queues, :success_codes, :error_codes, :render_pass, :command_buffer_levels, :comment, :return_type, :name, :parameters, :validity, :implicit_external_sync_parameters
    end

    class Constant < Vulkan::Normalized::Object
      representation :guard, :name, :value
    end

    class Define < Vulkan::Basic::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :text
    end

    class Enum < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :range, :vendor, :values
    end

    class EnumValue < Vulkan::Normalized::Object
      representation :guard, :name, :value, :bit_position, :api, :type, :name_alias
    end

    class Extension < Vulkan::Normalized::Object
      representation :name, :number, :supported, :protect, :author, :contact, :comment, :blocks
    end

    class Feature < Vulkan::Normalized::Object
      representation :api, :name, :number, :blocks, :types, :commands
    end

    class Function < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :text, :types
    end

    class FunctionPointer < Vulkan::Normalized::Object
      representation :type, :optional
    end

    class Handle < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :dispatchable
    end

    class Include < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :text
    end

    class Member < Vulkan::Normalized::Object
      representation :no_auto_validity, :external_sync, :type, :name
    end

    class Pointer < Vulkan::Normalized::Object
      representation :type, :length, :mutable, :optional
    end

    class Primitive < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only
    end

    class Require < Vulkan::Normalized::Object
      representation :guard, :api, :comment, :name, :requires
    end

    class Struct < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :members, :validity
    end

    class Union < Vulkan::Normalized::Object
      representation :guard, :requires, :name, :api, :comment, :parents, :returned_only, :members, :validity
    end

    class Builder < Vulkan::Builder
      def initialize(basic_ast, features: nil, extensions: nil)
        @basic_ast = basic_ast

        @features, @extensions = features, extensions

        raise ArgumentError, 'not a basic ast' unless @basic_ast.is_a? Vulkan::Basic::AST

        super()
      end

      def build_ast
        features, extensions, metadata = [], [], []

        enums, types, commands = {}, {}, {}

        children = @basic_ast.children.map do |child|
          case child
          when Vulkan::Basic::Comment, Vulkan::Basic::VendorIdentifiers, Vulkan::Basic::Tags
            metadata << child
          when Vulkan::Basic::Enums
            enums[child.name] = child
          when Vulkan::Basic::Types
            child.types.each do |type|
              types[type.name] = type
            end
          when Vulkan::Basic::Commands
            child.commands.each do |command|
              commands[command.name] = command
            end
          when Vulkan::Basic::Feature
            if @features == nil || @features.include?(child.name)
              features << [Feature.new(api: child.api, name: child.name, number: child.number, blocks: [], types: [], commands: []), child]
            end
          when Vulkan::Basic::Extensions
            child.extensions.map do |extension|
              if @extensions == nil || @extensions.include?(extension.name)
                extensions << [Extension.new(name: extension.name, number: extension.number, supported: extension.supported, protect: extension.protect, author: extension.author, contact: extension.contact, comment: extension.comment, blocks: []), extension]
              end
            end
          else
            raise ArgumentError, "basic ast contains weird things"
          end
        end.flatten(1)

        normalized_types, normalized_enums = build_types(types, enums)
        normalized_commands = build_commands(commands, normalized_types)

        features.each do |feature, basic_feature|
          feature_children = basic_feature.children.map do |child|
            case child
            when Vulkan::Basic::Require
              block_children = child.children.map do |block_child|
                value = case block_child[:type]
                when 'command'
                  command = normalized_commands.fetch(block_child[:name])

                  command.parameters.each do |param|
                    mark_type(param.type, feature)
                  end

                  mark_type(command.return_type, feature)

                  command
                when 'enum'
                  normalized_enums.fetch(block_child[:name])
                when 'type'
                  normalized_types.fetch(block_child[:name])
                else
                  raise ArgumentError, "#{block_child[:type]}"
                end

                raise ArgumentError, "already has a guard #{value.guard}" if value.guard && value.guard != feature

                value.guard = feature
                value
              end

              Block.new(comment: child.comment, children: block_children)
            else
              raise ArgumentError
            end
          end

          feature.blocks = feature_children
          feature.types = normalized_types.values.select { |x| x.guard == feature }
          feature.commands = normalized_commands.values.select { |x| x.guard == feature }
        end

        AST.new(features: features.map(&:first), extensions: extensions.map(&:first), metadata: metadata)
      end

      def build_commands(commands, types)
        normalized_commands = {}

        commands.values.each do |command|
          children = command.children.group_by(&:class)

          raise ArgumentError unless children.keys.all? { |x| [Vulkan::Basic::Parameter, Vulkan::Basic::Validity, Vulkan::Basic::ImplicitExternSyncParams].include? x }

          return_type = types.fetch(command.return_type)
          validity = children[Vulkan::Basic::Validity]
          implicit_external_sync = children[Vulkan::Basic::ImplicitExternSyncParams]

          parameters = children[Vulkan::Basic::Parameter].map do |member|
            build_param(member, Member, types)
          end

          normalized_commands[command.name] = Command.new(guard: nil, block: nil, queues: command.queues, success_codes: command.success_codes, error_codes: command.error_codes, render_pass: command.render_pass, command_buffer_levels: command.command_buffer_levels, comment: command.comment, return_type: return_type, name: command.name, parameters: parameters, validity: validity, implicit_external_sync_parameters: implicit_external_sync)
        end

        normalized_commands
      end

      def build_param(param, klass, normalized_types)
        new_type = normalized_types.fetch(param.type)

        if param.enum
          new_type = Array.new(type: new_type, length: param.enum)
        elsif param.name.underscore.start_with? 'pfn_'
          new_type = FunctionPointer.new(type: new_type, optional: param.optional)
        elsif param.name.underscore.start_with? 'pp_'
          raise ArgumentError if param.lengths && param.lengths.length != 2

          l1 = param.lengths ? param.lengths[1] : nil
          l0 = param.lengths ? param.lengths[0] : nil

          raise ArgumentError unless [0, 2].include? param.text.scan(/const/).length

          new_type = Pointer.new(type: new_type, length: l1, mutable: param.text.scan(/const/).length == 0, optional: false)
          new_type = Pointer.new(type: new_type, length: l0, mutable: param.text.scan(/const/).length == 0, optional: param.optional)
        elsif param.name.underscore.start_with? 'p_'
          length = param.lengths ? param.lengths[0] : nil
          mutable = param.text.scan(/const/).length < 1

          new_type = Pointer.new(type: new_type, length: length, mutable: mutable, optional: param.optional)
        end

        klass.new(name: param.name, type: new_type, external_sync: param.external_sync, no_auto_validity: param.no_auto_validity)
      end

      def build_types(types, enums)
        normalized_types, normalized_enums = {}, {}

        enums.values.select { |enum| enum.type.nil? }.each do |enum|
          enum.enums.each do |value|
            constant = Constant.new(guard: nil, name: value.name, value: value.value)
            normalized_enums[constant.name] = constant
            normalized_types[constant.name] = constant
          end
        end

        tys = Sorter.new(types).tsort.map { |n| types.fetch(n) }

        tys.each do |type|
          params = { guard: nil, requires: type.requires, name: type.name, api: type.api, comment: type.comment, parents: type.parents, returned_only: type.returned_only }

          case type.category
          when 'basetype'
            params[:type] = normalized_types.fetch(type.definition.type)
            normalized_types[type.name] = Alias.new(**params)
          when 'bitmask'
            params[:type] = normalized_types.fetch(type.definition.type)

            if enum = enums[type.requires]
              bit_mask_values = enum.enums.map do |value|
                if value.is_a? Vulkan::Basic::Enum
                  BitMaskValue.new(guard: nil, name: value.name, value: value.value, bit_position: value.bitpos, api: value.api, type: value.type, name_alias: value.name_alias).tap do |v|
                    normalized_enums[v.name] = v
                  end
                end
              end.compact

              params[:values] = bit_mask_values
            else
              params[:values] = []
            end

            normalized_types[type.name] = BitMask.new(**params)
          when 'define'
            params[:text] = type.definition.text
            normalized_types[type.name] = Define.new(**params)
          when 'funcpointer'
            params[:text] = type.definition.text
            params[:types] = type.definition.types

            normalized_types[type.name] = Function.new(**params)
          when 'handle'
            dispatchable = case type.definition.type
            when 'VK_DEFINE_HANDLE' then true
            when 'VK_DEFINE_NON_DISPATCHABLE_HANDLE' then false
            else
              raise ArgumentError, "unknown handle type #{type.definition.type.inspect}"
            end

            params[:dispatchable] = dispatchable

            normalized_types[type.name] = Handle.new(**params)
          when 'include'
            params[:text] = type.definition.text
            normalized_types[type.name] = Include.new(**params)
          when 'enum'
            enum = enums[type.name]

            if enum = enums[type.name]
              params[:comment] ||= enum.comment
              params[:range] = enum.range
              params[:vendor] = enum.vendor
              params[:values] = enum.enums

              enum_values = enum.enums.map do |value|
                if value.is_a? Vulkan::Basic::Enum
                  EnumValue.new(guard: nil, name: value.name, value: value.value, bit_position: value.bitpos, api: value.api, type: value.type, name_alias: value.name_alias).tap do |v|
                    normalized_enums[v.name] = v
                  end
                end
              end.compact

              params[:values] = enum_values
            else
              params[:range] = nil
              params[:vendor] = nil
              params[:values] = []
            end

            normalized_types[type.name] = Enum.new(**params)
          when 'struct', 'union'
            children = type.definition.children.group_by(&:class)

            raise ArgumentError unless children.keys.all? { |x| [Vulkan::Basic::Validity, Vulkan::Basic::Member].include? x }

            params[:validity] = children[Vulkan::Basic::Validity]
            params[:members] = children[Vulkan::Basic::Member].map do |member|
              build_param(member, Member, normalized_types)
            end

            normalized_types[type.name] = { struct: Struct, union: Union }.fetch(type.category.to_sym).new(**params)
          when nil
            normalized_types[type.name] = Primitive.new(**params)
          else
            raise ArgumentError, "unknown type category #{type.category}"
          end
        end

        [normalized_types, normalized_enums]
      end

      def mark_type(type, feature)
        if type.respond_to? :guard=
          type.guard = feature unless type.guard

          if type.respond_to? :members
            type.members.each do |member|
              mark_type(member.type, feature)
            end
          end

          if type.respond_to? :values
            type.values.each do |value|
              value.guard = feature unless value.guard
            end
          end
        end

        if type.respond_to? :type
          mark_type(type.type, feature)
        end
      end
    end
  end
end
