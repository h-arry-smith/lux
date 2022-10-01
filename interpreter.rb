require_relative "fixture"
require_relative "ast"
require_relative "world"
require_relative "selection_engine"
require_relative "time_context"
require_relative "value"
require_relative "value_range"
require_relative "value_sequence"
require_relative "query_builder"
require_relative "function_registry"

class Interpreter
  attr_reader :world
  
  def initialize(ast)
    @ast = ast
    @world = World.new()
    @current_param = nil

    # Temporary World
    6.times { |x| @world.add(Dimmer.new(x + 1, 1, 1 + x)) }
    6.times { |x| @world.add(MovingLight.new(x + 101, 1, 101+(6*x))) }
    
    @selection_engine = SelectionEngine.new()
    @query_builder = QueryBuilder.new()
    @functions = FunctionRegister
    @globals = {}
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

  def visit_call(expr)
    args = expr.arguments.map { |argument| evaluate(argument) }
    args = args.flat_map { |argument| generate_value(argument) }

    @functions.get(expr.identifier).call(*args)
  end

  def visit_selector(expr)
    evaluate(expr.selector)
  end

  def visit_block(expr)
    expr.statements.each { |statement| evaluate(statement) }
  end

  def visit_apply(expr)
    @current_param = expr.parameter
    value = expr.value.map { |val| evaluate(val) }
    value = value.flat_map { |val| generate_value(val, expr.parameter) }

    if value.length == 1
      value = value.first
    else
      value = ValueSequence.new(value)
    end

    @world.fixtures.each { |fixture| fixture.apply(expr.parameter, value&.get, @world.time_context) }
    @current_param = nil
  end

  def generate_value(value, parameter = nil)
    case value
    when Numeric
      StaticValue.new(value)
    when Range
      ValueRange.new(value.first, value.last, @world.fixtures.length)
    when Hash
      ValueTuple.new(value)
    else
      value
    end
  end

  def visit_tuple(expr)
    tuple = {}

    expr.values.each { |k, v| tuple[k] = generate_value(evaluate(v)) }

    tuple
  end

  def visit_value(expr)
    expr.value
  end

  def visit_range(expr)
    start = evaluate(expr.start)
    finish = evaluate(expr.end)

    # If you try to range between multiple values that will fallback to
    # taking the first value in the sequence, because this behaviour is
    # unusual and not really supported.
    start = start.first if start.is_a? Array
    finish = finish.first if finish.is_a? Array
    
    (start..finish)
  end

  def visit_vardefine(expr)
    @globals[expr.identifier] = expr.block
  end

  def visit_varfetch(expr)
    if @current_param
      get_value_from_block(@globals[expr.identifier], @current_param)
    else
      evaluate(@globals[expr.identifier])
    end
  end

  def interpret
    @ast.each { |statement| evaluate(statement) }
  end

  private

  def evaluate(expr)
    expr.accept(self)
  end

  def get_value_from_block(block, parameter)
    statement = block.statements.find do |statement|
      statement.is_a?(Ast::Apply) && statement.parameter == parameter
    end

    statement.value.map { |val| evaluate(val) } unless statement.nil?
  end

  def select_fixtures(query)
    @world = @selection_engine.select(@world, query)
  end

  def deselect_fixtures
    @world = @world.deselect
  end
end
