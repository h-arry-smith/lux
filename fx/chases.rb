module FX
  class << self
    def cycle(time, period, *values)
      index = (time % period).round
      p index

      values[index]
    end
  end
end
