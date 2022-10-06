require_relative "value"

module Core
  class Delay < Value
    attr_reader :time

    def initialize(start, finish, time)
      @start = start
      @finish = finish
      @time = time
      @run_from = nil
    end

    def value
      @finish
    end

    def get
      self
    end

    def run(elapsed_time)
      unless @run_from.nil?
        elapsed_time += @run_from
      end

      if elapsed_time < @time
        return @start.run(elapsed_time)
      end

      # We subtract the delay time here because we would want a fade to act as
      # if it had just started, in essence delaying it's 0s mark forward in
      # time
      @finish.run(elapsed_time - @time)
    end

    def to_s
      "#{@start} @#{@time}s #{@finish}"
    end

    def resolve(elapsed_time)
      result = self

      if elapsed_time >= @time
        if @finish.is_a?(Fade) || @finish.is_a?(Delay)
          result = @finish.resolve(elapsed_time)
        else
          result = @finish
        end
      end

      @run_from = elapsed_time

      result
    end

    def self.from(current, target, time_context)
      target = current if target.nil?
      return current if target.value == current.value && !time_context.any_delay?
      return target unless time_context.any_delay?

      if time_context.dup && target.value > current.value
        return Delay.new(current, target, time_context.dup)
      end

      if time_context.ddown && target.value < current.value
        return Delay.new(current, target, time_context.ddown)
      end

      return Delay.new(current, target, time_context.delay)
    end
  end
end
