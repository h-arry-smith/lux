require_relative "ast_printer"
require_relative "lexer"
require_relative "parser"

puts "Lux Interpreter"
puts "Enter '.exit' to exit"

input = ""

while true
  print "> "
  input = gets.chomp

  break if input == ".exit"

  lexer = Lexer.new(input)
  lexer.scan_tokens

  puts "# TOKENS #"

  lexer.tokens.each { |token| puts token }

  parser = Parser.new(lexer.tokens)

  ast = parser.parse
  ast_printer = AstPrinter.new

  puts "# AST #"

  ast_printer.print_tree(ast)
end
