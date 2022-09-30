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

  def initialize(id, default, offset)
    @id = id
    @default = StaticValue.new(default)
    @offset = offset
  end

  def to_dmx(current_value)
    ((current_value / 100.0) * 255).round
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
