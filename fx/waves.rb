module FX
  class << self
    def sin(time, min = 0, max = 100, period = 1, phase = 0)
      difference = max - min
      factor = (Math.sin((time / period) - phase) + 1) / 2.0

      min + (difference * factor)
    end
  end
end
