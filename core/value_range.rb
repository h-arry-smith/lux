require_relative "value"

module Core
  class ValueRange < Value
    attr_reader :start, :finish
    def initialize(start, finish, total)
      @start = start.value
      @finish = finish.value
      @type = start.class
      @total = total - 1

      @current = 0
      @step = ((@finish-@start) / @total)
    end

    def get
      return @type.new(@finish) if @current == @total

      value = @start + (@step * @current)
      @current += 1

      @type.new(value.round(2))
    end
  end
end
