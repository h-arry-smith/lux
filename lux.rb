require_relative "ast_printer"
require_relative "interpreter"
require_relative "lexer"
require_relative "parser"
require_relative "lighting_engine"
require_relative "output"
require_relative "timer"

class Lux
  def initialize(debug_flags)
    @debug_flags = debug_flags
    @lighting_engine = LightingEngine.new()
    @time = Timer.new()

    @output = SACNOutput.new("127.0.0.1")
    @output.connect()
  end

  def run(world)
    @time.start

    loop do
      @time.delta_start
      
      @lighting_engine.run(world, @time.elapsed)

      if @debug_flags[:dump_universe]
        @lighting_engine.dump_universes
      end

      if @debug_flags[:broadcast]
        @output.broadcast(@lighting_engine.universes)
      end

      sleep(@time.target_hz(20))
    end
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
