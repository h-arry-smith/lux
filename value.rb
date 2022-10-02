class Value
  def get
    raise NotImplementedError
  end

  def run(_time)
    raise NotImplementedError
  end

  def to_s
    "Value"
  end
end

class StaticValue < Value
  attr_reader :value

  def initialize(value)
    @value = value
  end

  def get
    self
  end

  def run(_time)
    @value
  end

  def >(other)
    @value > other.value
  end

  def <(other)
    @value < other.value
  end

  def to_s
    "StaticValue(#{@value})"
  end
end

class ValueTuple < Value
  def initialize(values)
    @values = values
  end

  def value
    @values
  end

  def get
    ValueTuple.new(@values.transform_values { |v| v&.get })
  end

  def run(time)
    @values 
  end

  def to_s
    "ValueTuple { #{@values} }"
  end

  def name_as(tuple)
    index = 0
    @values.transform_keys! do
      key = tuple.value.keys[index]
      index += 1
      key
    end
    self
  end

  def named_tuple?
    @values.all? { |k, _| !k.to_s.start_with?("_") }
  end

  def anonymous_tuple?
    @values.all? { |k, _| k.to_s.start_with?("_") }
  end
end
