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

  def run(elapsed_time)
    start_value = @start.run(elapsed_time)
    finish_value = @finish.run(elapsed_time)
    change = finish_value - start_value
    
    if elapsed_time == 0
      return start_value
    end

    if elapsed_time > @time
      return finish_value
    end

    time_factor = elapsed_time / @time
    return start_value + (change * time_factor)
  end

  def value
    @finish.value
  end

  def to_s
    "Fade(#{@start}->#{@finish}, #{@time}s)"
  end

  def self.from(current, target, time_context)
    if current.is_a?(ValueTuple) && target.is_a?(ValueTuple)
      hash_with_fades = {}

      puts "current: #{current}"

      target.name_as(current) if target.anonymous_tuple?

      puts "target: #{target}"

      current.value.each do |parameter, current_value|
        fade = self.from(current_value, target.value[parameter], time_context)
        hash_with_fades[parameter] = fade
      end

      return ValueTuple.new(hash_with_fades)
    end

    return target unless time_context.any_fade?
    target = current if target.nil?

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
