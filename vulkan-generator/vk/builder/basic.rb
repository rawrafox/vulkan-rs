module Vulkan
  module Basic
    class Object < Vulkan::Object
    end

    class AST < Vulkan::Basic::Object
      representation :children
    end

    class BaseType < Vulkan::Basic::Object
      representation :type
    end

    class BitMask < Vulkan::Basic::Object
      representation :type
    end

    class Command < Vulkan::Basic::Object
      representation :queues, :success_codes, :error_codes, :render_pass, :command_buffer_levels, :comment, :return_type, :name, :children
    end

    class Commands < Vulkan::Basic::Object
      representation :commands
    end

    class Comment < Vulkan::Basic::Object
      representation :text
    end

    class Constant < Vulkan::Basic::Object
      representation :name, :value
    end

    class Define < Vulkan::Basic::Object
      representation :text
    end

    class Enums < Vulkan::Basic::Object
      representation :name, :type, :range, :vendor, :comment, :enums
    end

    class Enum < Vulkan::Basic::Object
      representation :name, :value, :bitpos, :api, :type, :name_alias
    end

    class Extension < Vulkan::Basic::Object
      representation :name, :number, :supported, :protect, :author, :contact, :comment, :children
    end

    class Extensions < Vulkan::Basic::Object
      representation :extensions
    end

    class Feature < Vulkan::Basic::Object
      representation :api, :name, :number, :children
    end

    class FunctionPointer < Vulkan::Basic::Object
      representation :text, :types
    end

    class Handle < Vulkan::Basic::Object
      representation :type
    end

    class ImplicitExternSyncParams < Vulkan::Basic::Object
      representation :parameters
    end

    class Include < Vulkan::Basic::Object
      representation :text
    end

    class Member < Vulkan::Basic::Object
      representation :lengths, :optional, :no_auto_validity, :external_sync, :type, :name, :enum, :text
    end

    class Parameter < Vulkan::Basic::Object
      representation :lengths, :optional, :no_auto_validity, :external_sync, :type, :name, :enum, :text
    end

    class Require < Vulkan::Basic::Object
      representation :profile, :comment, :api, :children
    end

    class Struct < Vulkan::Basic::Object
      representation :children
    end

    class Tag < Vulkan::Basic::Object
      representation :name, :author, :contact
    end

    class Tags < Vulkan::Basic::Object
      representation :tags
    end

    class Types < Vulkan::Basic::Object
      representation :types
    end

    class Type < Vulkan::Basic::Object
      representation :requires, :name, :api, :category, :comment, :parents, :returned_only, :definition
    end

    class Union < Vulkan::Basic::Object
      representation :children
    end

    class Unused < Vulkan::Basic::Object
      representation :range, :vendor, :comment
    end

    class Validity < Vulkan::Basic::Object
      representation :usage
    end

    class VendorIdentifier < Vulkan::Basic::Object
      representation :name, :identifier, :comment
    end

    class VendorIdentifiers < Vulkan::Basic::Object
      representation :identifiers
    end

    class Filler < Vulkan::Basic::Object
      representation :filler
    end

    class Builder < Vulkan::Builder
      def initialize(xml)
        @xml = GeneratorNode.new(xml)

        super()
      end

      def build_ast
        children = dispatch(@xml.at('/registry'))

        AST.new(children: children)
      end

      def dispatch(node, prefix: nil, skip: 0)
        result = node.children[skip .. -1].map do |child|
          method = "build_#{"#{prefix}_" if prefix}#{child.name}".to_sym
          self.send(method, child)
        end

        raise ArgumentError unless result.all? { |x| x.is_a? Vulkan::Basic::Object }

        result
      end

      def new_param(param, klass)
        lengths = param[:len] ? param[:len].split(',') : nil

        klass.new(lengths: lengths, optional: param[:optional], no_auto_validity: param[:noautovalidity], external_sync: param[:externsync], type: param.string('./type'), name: param.string('./name'), enum: param.string('./enum', optional: true), text: param.inner_text)
      end

      def build_commands(commands)
        Commands.new(commands: dispatch(commands, prefix: 'commands'))
      end

      def build_commands_command(command)
        proto = command.children.first

        raise ArgumentError, 'first child of command is not a proto' unless proto.name == 'proto'

        command_buffer_levels = command[:cmdbufferlevel] ? command[:cmdbufferlevel].split(',') : nil

        Command.new(queues: command[:queues], success_codes: command[:successcodes], error_codes: command[:errorcodes], render_pass: command[:renderpass], command_buffer_levels: command_buffer_levels, comment: command[:comment], name: proto.string('./name'), return_type: proto.string('./type'), children: dispatch(command, prefix: 'commands_command', skip: 1))
      end

      def build_commands_command_param(param)
        new_param(param, Parameter)
      end

      def build_commands_command_validity(validity)
        Validity.new(usage: validity.children.map(&:inner_text))
      end

      def build_commands_command_implicitexternsyncparams(sync)
        ImplicitExternSyncParams.new(parameters: sync.children.map(&:inner_text))
      end

      def build_comment(comment)
        Comment.new(text: comment.inner_text)
      end

      def build_enums(enums)
        range = if enums[:start] || enums[:end]
          enums[:start] .. enums[:end]
        end

        Enums.new(name: enums[:name], type: enums[:type], range: range, vendor: enums[:vendor], comment: enums[:comment], enums:  dispatch(enums, prefix: 'enums'))
      end

      def build_enums_enum(enum)
        Enum.new(name: enum[:name], value: enum[:value], bitpos: enum[:bitpos], api: enum[:api], type: enum[:type], name_alias: enum[:alias])
      end

      def build_enums_unused(unused)
        range = if unused[:end]
          unused[:start] .. unused[:end]
        else
          unused[:start]
        end

        Unused.new(range: range, vendor: unused[:vendor], comment: unused[:comment])
      end

      def build_extensions(extensions)
        Extensions.new(extensions: dispatch(extensions, prefix: 'extensions'))
      end

      def build_extensions_extension(extension)
        Extension.new(name: extension[:name], number: extension[:number], supported: extension[:supported], protect: extension[:protect], author: extension[:author], contact: extension[:contact], comment: extension[:comment], children: dispatch(extension, prefix: 'feature_or_extension'))
      end

      def build_feature(feature)
        Feature.new(api: feature[:api], name: feature[:name], number: feature[:number], children: dispatch(feature, prefix: 'feature_or_extension'))
      end

      def build_feature_or_extension_require(req)
        children = req.children.map do |child|
          result = { type: child.name }

          result[:comment] = child[:comment] if child[:comment]

          case child.name
          when 'command', 'type'
            result[:name] = child[:name]
          when 'enum'
            result[:name] = child[:name]
            result[:value] = child[:value] if child[:value]
            result[:bitpos] = child[:bitpos] if child[:bitpos]
            result[:extends] = child[:extends] if child[:extends]
            result[:offsets] = child[:offsets] if child[:offsets]
            result[:dir] = child[:dir] if child[:dir]
          else
            raise ArgumentError, "unsupported type #{child.name}"
          end

          result
        end

        Require.new(profile: req[:profile], comment: req[:comment], api: req[:api], children: children)
      end

      def build_tags(tags)
        Tags.new(tags: dispatch(tags, prefix: 'tags'))
      end

      def build_tags_tag(tag)
        Tag.new(name: tag[:name], author: tag[:author], contact: tag[:contact])
      end

      def build_types(types)
        Types.new(types: dispatch(types, prefix: 'types'))
      end

      def build_types_type(type)
        definition = case type[:category]
        when 'basetype'
          BaseType.new(type: type.string('./type'))
        when 'bitmask'
          BitMask.new(type: type.string('./type'))
        when 'define'
          Define.new(text: type.inner_text)
        when 'funcpointer'
          FunctionPointer.new(text: type.inner_text, types: type.map('./type', &:inner_text))
        when 'handle'
          Handle.new(type: type.string('./type'))
        when 'include'
          Include.new(text: type.inner_text)
        when 'struct'
          Struct.new(children: dispatch(type, prefix: 'types_type'))
        when 'union'
          Union.new(children: dispatch(type, prefix: 'types_type'))
        when nil, 'enum'
          nil
        else
          raise ArgumentError, "unsupported category #{type[:category]}"
        end

        Type.new(requires: type[:requires], name: type[:name] || type.string('./name'), api: type[:api], category: type[:category], comment: type[:comment], parents: (type[:parent] || '').split(','), returned_only: type[:returnedonly], definition: definition)
      end

      def build_types_type_member(member)
        new_param(member, Member)
      end

      alias_method :build_types_type_validity, :build_commands_command_validity

      def build_vendorids(vendorids)
        VendorIdentifiers.new(identifiers: dispatch(vendorids, prefix: 'vendorids'))
      end

      def build_vendorids_vendorid(vendor_id)
        VendorIdentifier.new(name: vendor_id[:name], identifier: vendor_id[:id], comment: vendor_id[:comment])
      end
    end
  end
end
