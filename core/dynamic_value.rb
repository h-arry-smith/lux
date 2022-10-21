require_relative "value"

module Core
  class DynamicValue < Value
    def initialize(function, fixture_count, arguments)
      super
      @function = function
      @fixture_count = fixture_count
      @arguments = arguments
    end

    def get
      # this handle arguments wih ranges by making sure when we get, we return
      # a version with those resolved
      return DynamicValue.new(@function, @fixture_count, @arguments.map { |arg| arg&.get })
    end

    def resolve_nil_values(value)
      @arguments.map! do |arg|
        if arg.nil?
          value
        else
          arg
        end
      end
    end

    # A dynamic value should always be faded or delayed to, set it returns some
    # ridiculous number to trigger fade/delays
    def value
      99999
    end

    def run(time)
      result = @function.call(context(time), *run_arguments(time))

      if result.is_a?(DynamicValue)
        result.run(time)
      else
        result
      end
    end

    def run_arguments(time)
      @arguments.map do |argument|
        if argument.is_a?(Value)
          argument.run(time)
        else
          argument
        end
      end
    end

    def context(time)
      {
        time: time,
        count: @fixture_count
      }
    end

    def to_s
      "< #{@function.name}(#{@arguments.join(", ")}) >"
    end
  end
end
