require_relative "lux"
require_relative "lib/sacn"

DEBUG_FLAGS = {
  token: false,
  ast: true,
  lx_state: true,
  dump_universe: false,
  broadcast: true
}

IP = "127.0.0.1"

lux = Lux.new(DEBUG_FLAGS)

if ARGV.empty?
  puts "Lux Interpreter"
  puts "Enter '.exit' to exit"

  while true
    print "> "
    input = gets.chomp

    break if input == ".exit"

    lux.evaluate(input)
  end
end

if ARGV.length == 1
  input = File.read(ARGV[0])

  world = lux.evaluate(input)

  engine = lux.run(world)
end
