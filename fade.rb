require_relative "value"

class Fade < Value
  attr_reader :start, :finish, :attr_reader
  
  def initialize(start, finish, time)
    @start = start
    @finish = finish
    @time = time
  end

  def get
    self
  end

  def to_s
    "Fade(#{@start}->#{@finish}, #{@time}s)"
  end

  def self.from(current, target, time_context)
    return current if current.value == target.value

    if time_context.fade?
      return Fade.new(current, target, time_context.fade)
    end

    target
  end
end
