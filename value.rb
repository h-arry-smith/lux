class Value
  def get
    raise NotImplementedError
  end

  def to_s
    "Value"
  end
end

class StaticValue < Value
  def initialize(value)
    @value = value
  end

  def get
    self
  end

  def to_s
    "StaticValue(#{@value})"
  end
end
