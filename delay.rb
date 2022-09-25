require_relative "value"

class Delay < Value
  attr_reader :time

  def initialize(start, finish, time)
    super()
    @start = start
    @finish = finish
    @time = time
  end

  def get
    self
  end

  def to_s
    "Delay @0s #{@start} -> @#{@time}s #{@finish}"
  end

  def self.from(current, target, time_context)
    return target unless time_context.any_delay?

    if time_context.dup && target.value > current.value
      return Delay.new(current, target, time_context.dup)
    end

    if time_context.ddown && target.value < current.value
      return Delay.new(current, target, time_context.ddown)
    end

    Delay.new(current, target, time_context.delay)
  end
end