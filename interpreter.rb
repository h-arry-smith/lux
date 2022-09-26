require_relative "fixture"
require_relative "world"
require_relative "selection_engine"
require_relative "time_context"
require_relative "value"
require_relative "value_range"
require_relative "value_sequence"
require_relative "query_builder"

class Interpreter
  attr_reader :world
  
  def initialize(ast)
    @ast = ast
    @world = World.new()

    # Temporary World
    6.times { |x| @world.add(Dimmer.new(x + 1)) }
    6.times { |x| @world.add(MovingLight.new(x + 101)) }
    
    @selection_engine = SelectionEngine.new()
    @query_builder = QueryBuilder.new()
  end

  def visit_selection(expr)
    selector = evaluate(expr.selector)

    query = @query_builder.build(selector)
    select_fixtures(query)

    evaluate(expr.block)

    deselect_fixtures
  end

  def visit_timeblock(expr)
    @world.push_time_context(TimeContext.new)

    expr.time.map { |time| evaluate(time) }

    evaluate(expr.block)

    @world.pop_time_context()
  end

  def visit_time(expr)
    if expr.keyword == Token::SNAP
      @world.time_context.set_to_snap
    else
      @world.time_context[expr.keyword] = expr.value
    end
  end

  def visit_selector(expr)
    evaluate(expr.selector)
  end

  def visit_block(expr)
    expr.statements.each { |statement| evaluate(statement) }
  end

  def visit_apply(expr)
    value = expr.value.map { |value| evaluate(value) }
    value = value.map { |value| generate_value(value) }

    if value.length == 1
      value = value.first
    else
      value = ValueSequence.new(value)
    end

    @world.fixtures.each { |fixture| fixture.apply(expr.parameter, value.get(), @world.time_context) }
  end

  def generate_value(value)
    if value.is_a?(Numeric)
      return StaticValue.new(value)
    elsif value.is_a?(Range)
      return ValueRange.new(value.first, value.last, @world.fixtures.length)
    elsif value.is_a?(Hash)
      value.each { |k, v| value[k] = generate_value(v) }
      return ValueTuple.new(value)
    end
  end

  def visit_tuple(expr)
    tuple = {}

    expr.values.each { |k, v| tuple[k] = evaluate(v) }

    tuple
  end

  def visit_value(expr)
    expr.value
  end

  def visit_range(expr)
    (expr.start..expr.end)
  end

  def interpret
    @ast.each { |statement| evaluate(statement) }
  end

  private

  def evaluate(expr)
    expr.accept(self)
  end

  def select_fixtures(query)
    @world = @selection_engine.select(@world, query)
  end

  def deselect_fixtures
    @world = @world.deselect
  end
end
