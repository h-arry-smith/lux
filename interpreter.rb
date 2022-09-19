require_relative "fixture"
require_relative "world"
require_relative "selection_engine"

class Interpreter
  attr_reader :world
  
  def initialize(ast)
    @ast = ast
    @world = World.new()

    # Temporary World
    3.times { |x| @world.add_fixture(Dimmer.new(x + 1)) }
    
    @selection_engine = SelectionEngine.new()
  end

  def visit_selection(expr)
    selector = evaluate(expr.selector)

    select_fixtures(selector)

    evaluate(expr.block)

    deselect_fixtures
  end

  def visit_selector(expr)
    evaluate(expr.selector)
  end

  def visit_block(expr)
    expr.statements.each { |statement| evaluate(statement) }
  end

  def visit_apply(expr)
    value = evaluate(expr.value)

    @world.fixtures.each { |fixture| fixture.apply(expr.parameter, value) }
  end

  def visit_value(expr)
    expr.value
  end

  def interpret
    @ast.each { |statement| evaluate(statement) }
  end

  private

  def evaluate(expr)
    expr.accept(self)
  end

  def select_fixtures(selector)
    @world = @selection_engine.select(@world, selector)
  end

  def deselect_fixtures
    @world = @world.deselect
  end
end
