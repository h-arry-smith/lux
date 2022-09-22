class TimeContext
  attr_reader :fade
  attr_writer :parent

  def initialize(parent = nil)
    @parent = parent

    @fade = nil
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

  def fade?
    if fade.nil?
      return @parent.fade? unless @parent.nil?
      return false
    end

    return true
  end
end
