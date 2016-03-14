module Vulkan
  class Object
    def self.representation(*attributes)
      initializer = ""
      initializer << "def initialize(#{attributes.map { |x| "#{x}:" }.join(', ')})\n"
      initializer << "  #{attributes.map { |x| "@#{x}" }.join(', ')} = #{attributes.join(', ')}\n"
      initializer << "end"

      class_eval(initializer)

      define_method(:as_json) do |o|
        result = { type: self.class.name.underscore }

        attributes.each do |attribute|
          value = self.send(attribute)

          result[attribute] = value.as_json(o) if value != nil
        end

        result
      end

      attr_accessor *attributes
    end
  end
end
