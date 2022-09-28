require "ruby-enum"

class ColorSpace
  include Ruby::Enum

  define :rgb, [:red, :green, :blue]
  define :cmy, [:cyan, :magenta, :yellow]
end
