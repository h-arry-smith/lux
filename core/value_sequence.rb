require_relative "value"

module Core
  class ValueSequence < Value
    def initialize(values)
      @values = values
      @current = 0
    end

    def get
      current_value = @values[@current]
      next_value

      current_value&.get
    end

    private

    def next_value
      @current += 1
      @current = 0 if @current >= @values.length
    end
  end
end
