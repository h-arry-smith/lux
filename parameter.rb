require_relative "value"

class GroupParameterInstance
  attr_accessor :parameter, :value

  def initialize(parameter)
    @parameter = parameter
    instantiate_children
    @value = default_tuple_from_children
    @time_context = nil
  end

  def apply(new_value, time_context)
    new_value = Fade.from(@value, new_value, time_context)
    new_value = Delay.from(@value, new_value, time_context)

    @value = new_value
  end

  def run(time)
    value = @value.run(time)

    if named_tuple? value
      value.each do |id, value|
        @children[id].value = value
      end
    end

    if anonymous_tuple? value
      index = 0
      @children.each do |_, child|
        new_value = value[:"_#{index}"]
        child.value = new_value unless new_value.nil?
        index += 1
      end
    end

    @children.map { |_, child| child.run(time) }
  end

  def offset
    @parameter.offset
  end

  private

  def named_tuple?(values)
    values.all? { |k, _| !k.to_s.start_with?("_") }
  end

  def anonymous_tuple?(values)
    values.all? { |k, _| k.to_s.start_with?("_") }
  end

  def instantiate_children
    @children = {}
    @parameter.children.each { |id, parameter| @children[id.to_sym] = ParameterInstance.new(parameter) }
  end

  def default_tuple_from_children
    defaults = {}
    p @children
    @children.each { |id, child| defaults[id.to_sym] = child.value }
    ValueTuple.new(defaults)
  end
end

class ParameterInstance
  attr_accessor :parameter, :value

  def initialize(parameter)
    @parameter = parameter
    @value = parameter.default
  end

  def apply(new_value, time_context)
    return if new_value.nil?
    new_value = Fade.from(@value, new_value, time_context)
    new_value = Delay.from(@value, new_value, time_context)

    @value = new_value
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

  def instantiate
    ParameterInstance.new(self)
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
  attr_reader :id, :children, :offset

  def initialize(id, offset)
    @id = id
    @children = {}
    @offset = offset
  end

  def instantiate
    GroupParameterInstance.new(self)
  end

  def []=(id, parameter)
    @children[id] = parameter
  end
end
