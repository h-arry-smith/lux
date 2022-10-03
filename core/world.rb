class World
  attr_reader :fixtures, :time_context
  
  def initialize(fixtures = [], parent = nil)
    @parent = parent
    @fixtures = fixtures

    if @parent.nil?
      @time_context = TimeContext.new
    else
      @time_context = parent.time_context
    end
  end

  # TODO : Any selection never has duplicates..
  def add(fixture)
    if fixture.is_a?(Array)
      @fixtures.concat(fixture)
    else
      @fixtures << fixture
    end
  end

  def reset
    @fixtures.each { |fixture| fixture.reset }
  end

  def push_time_context(context)
    context.parent = @time_context
    @time_context = context
  end

  def pop_time_context
    @time_context = @time_context.pop
  end

  def deselect
    return @parent
  end

  def get_time_context
  end
end
