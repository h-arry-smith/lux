require_relative "ast_printer"
require_relative "interpreter"
require_relative "lexer"
require_relative "parser"

class Lux
  def initialize(debug_flags)
    @debug_flags = debug_flags
  end

  def run(input)
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
  end
end
