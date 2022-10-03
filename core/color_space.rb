require "ruby-enum"
require_relative "value"

module Core
  class ColorSpace
    include Ruby::Enum

    define :rgb, [:red, :green, :blue]
    define :cmy, [:cyan, :magenta, :yellow]

    def self.convert(color, target_color_space)
      current_color_space = get_color_space(color)

      return color if current_color_space == target_color_space

      return method(:"#{current_color_space}_to_#{target_color_space}").call(color)
    end

    def self.get_color_space(values)
      key(values.keys)
    end

    def self.rgb_to_cmy(color)
      red, green, blue = color.values
      {
        cyan: StaticValue.new(100 - red.value),
        magenta: StaticValue.new(100 - green.value),
        yellow: StaticValue(100 - blue.value)
      }
    end

    def self.cmy_to_rgb(color)
      cyan, magenta, yellow = color.values
      {
        red: StaticValue.new(100 - cyan.value),
        green: StaticValue.new(100 - magenta.value),
        blue: StaticValue.new(100 - yellow.value),
      }
    end
  end
end
