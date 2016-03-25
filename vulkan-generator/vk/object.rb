module Vulkan
  class Object
    def initialize(args)
      self.reify!(**args)
    end

    def reify!
    end

    def self.representation(*attributes)
      define_singleton_method(:attributes) do
        super() + attributes
      end

      initializer = ""
      initializer << "def reify!(#{self.attributes.map { |x| "#{x}:" }.join(', ')})\n"
      initializer << "  super(#{self.superclass.attributes.map { |x| "#{x}: #{x}" }.join(', ')})\n"
      initializer << "  #{attributes.map { |x| "@#{x}" }.join(', ')} = #{attributes.join(', ')}\n"
      initializer << "end"

      class_eval(initializer)

      protected(:reify!)

      attr_accessor *attributes
    end

    def self.unserialized(*attributes)
      define_singleton_method(:unserialized_attributes) do
        super() + attributes
      end
    end

    def self.serialize(attribute, proc)
      define_singleton_method("#{attribute}_serializer".to_sym, proc)
    end

    def self.attributes
      []
    end

    def self.unserialized_attributes
      []
    end

    def serialize(attributes, method, *args)
      result = { _type: self.class.name.underscore }

      attributes.each do |attribute|
        value = self.send(attribute)

        serializer = "#{attribute}_serializer".to_sym
        value = self.class.send(serializer, value) if self.class.respond_to? serializer

        result[attribute] = value.send(method, *args) if value != nil
      end

      result
    end

    def as_json(o)
      serialize(self.class.attributes - self.class.unserialized_attributes, :as_json, o)
    end
  end
end
