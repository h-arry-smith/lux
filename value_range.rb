require_relative "value"

class ValueRange < Value
  def initialize(start, finish, total)
    @start = start
    @finish = finish
    @total = total - 1

    @current = 0
    @step = ((finish-start) / total)
  end

  def get
    return @finish if @current == @total

    value = @start + (@step * @current)
    @current += 1
    value.round(2)
  end
end
