require_relative "../value"
require_relative "../value_range"
require_relative "../dynamic_value"

module FX
  class << self
    def cycle(context, period, *values)
      time = context[:time]

      index = ((time / period) % values.length).floor

      values[index]
    end

    def chase(count, period = nil, low = nil, high = nil)
      period = Core::StaticValue.new(1) if period.nil?

      total_period = Core::StaticValue.new(count * period.value)

      phase_range = Core::ValueRange.new(
        0,
        total_period.value - period.value,
        count
      )

      args = [low, high, period, total_period, phase_range]

      Core::DynamicValue.new(method(:pulse), count, args)
    end
  end
end
