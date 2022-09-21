class Value
  def get
    raise NotImplementedError
  end
end

class StaticValue < Value
  def initialize(value)
    @value = value
  end

  def get
    @value
  end
end
