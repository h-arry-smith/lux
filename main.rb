require_relative "lux"

DEBUG_FLAGS = {
  token: true,
  ast: true,
  lx_state: true
}

lux = Lux.new(DEBUG_FLAGS)

if ARGV.empty?
  puts "Lux Interpreter"
  puts "Enter '.exit' to exit"

  while true
    print "> "
    input = gets.chomp

    break if input == ".exit"

    lux.run(input)
  end
end

if ARGV.length == 1
  input = File.read(ARGV[0])

  lux.run(input)
end
