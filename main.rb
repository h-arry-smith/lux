require "listen"
require_relative "lux"
require_relative "lib/sacn"

DEBUG_FLAGS = {
  token: false,
  ast: false,
  lx_state: false,
  dump_universe: false,
  broadcast: true
}

IP = "127.0.0.1"

lux = Lux.new(DEBUG_FLAGS)

listener = Listen.to('./', only: /\.lux$/) do |modified, added, removed|
  start = Time.now.to_f
  input = File.read(modified.first)
  lux.evaluate(input)
  lux.reload()
  finish = Time.now.to_f

  ms = ((finish-start)*1000).round
  puts "Reloading lighting state... #{ms}ms"
end

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
  listener.start
  input = File.read(ARGV[0])

  puts "Running #{ARGV[0]}..."

  lux.evaluate(input)

  lux.run()
end
