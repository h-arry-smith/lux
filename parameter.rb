require_relative "value"

class ParameterInstance
  attr_accessor :parameter, :value

  def initialize(parameter)
    @parameter = parameter
    @value = parameter.default
  end
end

class Parameter
  attr_reader :id, :default
  attr_accessor :value

  def initialize(id, default)
    @id = id
    @default = StaticValue.new(default)
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
