require_relative "ast_printer"
require_relative "interpreter"
require_relative "lexer"
require_relative "parser"
require_relative "lighting_engine"

class Lux
  def initialize(debug_flags)
    @debug_flags = debug_flags
    @lighting_engine = LightingEngine.new()
  end

  def run(world)
    @lighting_engine.run(world)
    if @debug_flags[:dump_universe]
      @lighting_engine.dump_universes
    end

    # Temporary, one day there will be a real event loop with UI but for now
    # we just want to get at the engine so we can broadcast it's universes
    @lighting_engine
  end

  def evaluate(input)
    lexer = Lexer.new(input)
    lexer.scan_tokens

    if @debug_flags[:token]
      puts "# TOKENS #"

      lexer.tokens.each { |token| puts token }
    end

    parser = Parser.new(lexer.tokens)

    ast = parser.parse

    if @debug_flags[:ast]
      puts "# AST #"

      ast_printer = AstPrinter.new
      ast_printer.print_ast(ast)
    end

    interpreter = Interpreter.new(ast)
    interpreter.interpret

    if @debug_flags[:lx_state]
      puts "# LIGHTING STATE #"
      interpreter.world.fixtures.each { |fixture| puts fixture }
    end

    return interpreter.world
  end
end
