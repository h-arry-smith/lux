module Core
  class TimeContext
    attr_writer :parent

    def initialize(parent = nil)
      @parent = parent

      @fade = nil
      @up = nil
      @down = nil
      @delay = nil
      @dup = nil
      @ddown = nil
    end

    def []=(keyword, value)
      if instance_variable_defined?("@#{keyword}")
        instance_variable_set("@#{keyword}", value)
      end
    end

    def set_to_snap
      @fade = 0
      @up = 0
      @down = 0
      @delay = 0
      @dup = 0
      @ddown = 0
    end

    def pop
      return self if @parent.nil?
      @parent
    end

    def fade
      get_time("fade")
    end

    def fade_up
      get_time("up")
    end

    def fade_down
      get_time("down")
    end

    def delay
      get_time("delay")
    end

    def dup
      get_time("dup")
    end

    def ddown
      get_time("ddown")
    end

    def get_time(keyword)
      time = nil
      if instance_variable_get("@#{keyword}").nil?
        time = @parent.get_time(keyword) unless @parent.nil?
      else
        time = instance_variable_get("@#{keyword}")
      end

      time
    end

    def any_delay?
      delay&.>(0) || dup&.>(0) || ddown&.>(0)
    end

    def any_fade?
      fade&.>(0) || fade_up&.>(0) || fade_down&.>(0)
    end
  end
end
