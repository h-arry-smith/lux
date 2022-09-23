class TimeContext
  attr_reader :fade, :up, :down
  attr_writer :parent

  def initialize(parent = nil)
    @parent = parent

    @fade = nil
    @up = nil
    @down = nil
  end

  def []=(keyword, value)
    if instance_variable_defined?("@#{keyword}")
      instance_variable_set("@#{keyword}", value)
    end
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

  def get_time(keyword)
    if instance_variable_get("@#{keyword}").nil?
      return @parent.get_time(keyword) unless @parent.nil?
      return nil
    end

    return instance_variable_get("@#{keyword}")
  end
end
