module Core
  class Timer
    attr_reader :time
    def initialize
      @time = current_time
      @delta = current_time
    end

    def start
      @time = current_time
    end

    def delta_start
      @delta = current_time
    end

    def target_hz(hz)
      elapsed = current_time - @delta
      target = 1.0 / hz

      target - elapsed
    end

    def elapsed
      current_time - @time
    end

    def current_time
      Time.now.to_f
    end
  end
end
