require_relative "value"

class ParameterInstance
  attr_accessor :value

  def initialize(parameter)
    @parameter = parameter
    @value = parameter.default
  end
end

class Parameter
  attr_reader :parameter, :default
  attr_accessor :value

  def initialize(parameter, default)
    @parameter = parameter
    @default = StaticValue.new(default)
  end

  def to_s
    "#{@parameter}"
  end
end

class GroupParameter
  attr_reader :parameter, :children

  def initialize(parameter)
    @parameter = parameter
    @children = []
  end

  def add_child_parameter(parameter)
    @children << parameter
  end
end
