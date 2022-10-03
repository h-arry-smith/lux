require_relative "value"

class DynamicValue < Value
  def initialize(function, arguments)
    @function = function
    @arguments = arguments
  end

  def get
    # this handle arguments wih ranges by making sure when we get, we return
    # a version with those resolved
    return DynamicValue.new(@function, @arguments.map { |arg| arg.get })
  end

  # A dynamic value should always be faded or delayed to, set it returns some
  # ridiculous number to trigger fade/delays
  def value
    99999
  end

  def run(time)
    ran_arguments = @arguments.map { |argument| argument.run(time) }
    @function.call(time, *ran_arguments)
  end

  def to_s
    "< #{@function.name}(#{@arguments.join(", ")}) >"
  end
end
