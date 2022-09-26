require_relative "value"

class Fade < Value
  attr_reader :start, :finish
  
  def initialize(start, finish, time)
    super()
    @start = start
    @finish = finish
    @time = time
  end

  def get
    self
  end

  def value
    @finish.value
  end

  def to_s
    "Fade(#{@start}->#{@finish}, #{@time}s)"
  end

  def self.from(current, target, time_context)
    return target unless time_context.any_fade?
    return current if target.nil? || current.value == target.value

    if time_context.fade_up && target > current
      return Fade.new(current, target, time_context.fade_up)
    end

    if time_context.fade_down && target < current
      return Fade.new(current, target, time_context.fade_down)
    end

    if time_context.fade
      return Fade.new(current, target, time_context.fade)
    end

    target
  end
end
