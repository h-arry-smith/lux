class Value
  def get
    raise NotImplementedError
  end

  def to_s
    "Value"
  end
end

class StaticValue < Value
  attr_reader :value

  def initialize(value)
    super()
    @value = value
  end

  def get
    self
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
