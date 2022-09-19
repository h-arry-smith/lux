require_relative "ast_printer"
require_relative "interpreter"
require_relative "lexer"
require_relative "parser"

DEBUG_TOKEN = false
DEBUG_AST = true
DEBUG_LX_STATE = true

puts "Lux Interpreter"
puts "Enter '.exit' to exit"

input = ""

while true
  print "> "
  input = gets.chomp

  break if input == ".exit"

  lexer = Lexer.new(input)
  lexer.scan_tokens

  if DEBUG_TOKEN
    puts "# TOKENS #"

    lexer.tokens.each { |token| puts token }
  end

  parser = Parser.new(lexer.tokens)

  ast = parser.parse

  if DEBUG_AST
    puts "# AST #"

    ast_printer = AstPrinter.new
    ast_printer.print_ast(ast)
  end

  interpreter = Interpreter.new(ast)
  interpreter.interpret

  if DEBUG_LX_STATE
    puts "# LIGHTING STATE #"
    interpreter.world.fixtures.each { |fixture| puts fixture }
  end
end
