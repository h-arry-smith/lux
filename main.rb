require_relative "lexer"

puts "Lux Interpreter"
puts "Enter '.exit' to exit"

input = ""

while true
  print "> "
  input = gets.chomp

  break if input == ".exit"

  lexer = Lexer.new(input)
  lexer.scan_tokens

  lexer.tokens.each { |token| puts token }
end
