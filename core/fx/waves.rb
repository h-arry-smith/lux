module FX
  class << self
    def sin(context, min = 0, max = 100, period = 1, phase = 0)
      difference = max - min
      period =  (Math::PI * 2) / period
      phase = phase * Math::PI
      factor = (Math.sin((context[:time] * period) - phase) + 1) / 2.0

      min + (difference * factor)
    end

    def square(context, min = 0, max = 100, period = 1, phase = 0)
      period =  (Math::PI * 2) / period
      phase = phase * Math::PI
      result = Math.sin((context[:time] * period) - phase)

      if result >= 0
        max
      else
        min
      end
    end

    def pulse(context, min = 0, max = 100, period = 1, total = 5, phase = 0)
      time = context[:time]
      time -= phase
      time = time % total

      if time <= period
        max
      else
        min
      end
    end
  end
end
