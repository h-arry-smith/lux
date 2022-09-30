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
    rand(@value)
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

  def get
    return @values
  end

  def to_s
    "Tuple { #{@values} }"
  end
end
