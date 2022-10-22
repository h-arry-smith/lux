require_relative "../value"

module FX
  class << self
    def circle(context)
      time = context[:time] * 90.0
      angle = time * (Math::PI / 180.0)
      
      x = Math::cos(angle) * 5.0
      y = Math::sin(angle) * 5.0
      
      values = [
        Core::PercentValue.new(50.0 + x),
        Core::PercentValue.new(50.0 + y)
      ]
      
      Core::ValueTuple.from_array(values)
    end
  end
end

# TODO: Circle / All shapes should be applied realtive to the 
#       position of the fixture, or whatever parameter it is being
#       applied too.