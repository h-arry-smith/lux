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

    if elapsed_time == 0
      return start_value
    end

    if elapsed_time > @time
      return finish_value
    end

    if start_value.is_a?(Numeric) && finish_value.is_a?(Numeric)
      numeric_fade(start_value, finish_value, elapsed_time)
    elsif start_value.is_a?(Hash) && finish_value.is_a?(Hash)
      hash_fade(start_value, finish_value, elapsed_time)
    else
      raise RuntimeError, "Unhandeld fade runtime case #{start_value.class} -> #{end_value.class}"
    end
  end

  def numeric_fade(start_value, finish_value, elapsed_time)
    change = finish_value - start_value
    time_factor = elapsed_time / @time
    start_value + (change * time_factor)
  end

  def hash_fade(start, finish, elapsed_time)
    fade_hash = {}

    start.length.times do |index|
      start_value = start.values[index].run(elapsed_time)
      finish_value = finish.values[index].run(elapsed_time)

      fade_hash[start.keys[index]] = StaticValue.new(numeric_fade(start_value, finish_value, elapsed_time))
    end

    fade_hash
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

      target.name_as(current) if target.anonymous_tuple?

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
