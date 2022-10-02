module FX
  class << self
    def cycle(time, period, *values)
      index = ((time / period) % values.length).floor

      values[index]
    end
  end
end
