module Vulkan
  class Name
    attr_reader :name, :type, :argument

    def initialize(name, type = :primitive, argument = nil)
      @name, @type, @argument = name, type, argument
    end

    def == (other)
      return false if self.class != other.class
      return self.name == other.name
    end

    def eql? (other)
      self == other
    end

    def hash
      self.name.hash
    end

    def to_s
      case self.type
      when :primitive then self.name
      when :command then self.name.sub(/^vk/, '').underscore
      when :extension then self.name.downcase
      when :constant then self.name.sub(/^VK_/, '')
      when :base_type, :bitfield, :enum, :handle, :struct then self.name.sub(/^Vk/, '')
      when :bitfield_member
        self.name.sub(/^VK_/, '')
      when :enum_member
        if self.argument && self.name.include?(self.argument)
          self.name.split(self.argument + '_')[1]
        else
          self.name.sub(/^VK_/, '')
        end.downcase.camelize
      when :struct_member then self.name
      when :function_pointer then self.name.sub(/^PFN_vk/, '') + 'Callback'
      else
        raise ArgumentError, "unknown type #{self.type.inspect}"
      end
    end

    def inspect
      "#<#{self.class.name} #{self.to_s.inspect}>"
    end
  end
end
