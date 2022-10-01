require_relative "value"

class ParameterInstance
  attr_accessor :parameter, :value

  def initialize(parameter)
    @parameter = parameter
    @value = parameter.default
  end

  def run(time)
    current_value = value.run(time)
    @parameter.to_dmx(current_value)
  end

  def offset
    @parameter.offset
  end
end

class Parameter
  attr_reader :id, :default, :offset
  attr_accessor :value

  def initialize(id, default, offset, min, max)
    @id = id
    @default = StaticValue.new(default)
    @offset = offset
    @min = min
    @max = max
  end

  def to_dmx(current_value)
    return 255 if current_value >= @max
    return 0 if current_value <= @min

    difference = @max - @min
    relative_value = current_value - @min
    factor = relative_value / difference
    

    (factor * 255).round
  end

  def to_s
    "#{@parameter}"
  end
end

class GroupParameter
  attr_reader :id, :children

  def initialize(id)
    @id = id
    @children = []
  end

  def add_child_parameter(parameter)
    @children << parameter
  end
end
