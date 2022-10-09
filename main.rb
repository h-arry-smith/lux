require "io/console"
require_relative "core/lux"
require_relative "console/console"

DEBUG_FLAGS = {
  token: false,
  ast: false,
  lx_state: false,
  dump_universe: false,
  dev_console: true,
  broadcast: true,
}

def temporary_console(lux)
  sleep(0.1)
  $stdout.clear_screen

  puts "Temporary Dev Console"
  puts "-"*80
  puts lux.loaded_cuelist
  puts "-"*80

  while true
    print "> "
    input = STDIN.gets
    input.chomp

    begin
      lux.evaluate(input)
      puts "-"*80
      puts lux.loaded_cuelist
      puts "-"*80
    rescue RuntimeError => e
      puts e
    end
  end
end

if ARGV.length == 1
  entry_file = ARGV[0]
  working_directory = File.dirname(File.expand_path(entry_file))

  lux = Core::Lux.new(working_directory, DEBUG_FLAGS)

  lux_thread = Thread.new { lux.start(entry_file) }

  if DEBUG_FLAGS[:dev_console]
    console = Console::Console.new(lux)
    console_thread = Thread.new { console.run() }
  end

  lux_thread.join
end

puts "Usage: lux <entry_file>"
