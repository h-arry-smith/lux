require "io/console"
require_relative "core/lux"
require_relative "console/console"

DEBUG_FLAGS = {
  token: false,
  ast: true,
  lx_state: true,
  dump_universe: false,
  dev_console: false,
  broadcast: false,
}

if ARGV.length == 1
  entry_file = ARGV[0]
  working_directory = File.dirname(File.expand_path(entry_file))

  lux = Core::Lux.new(working_directory, DEBUG_FLAGS)

  lux_thread = Thread.new { lux.start(entry_file) }

  if DEBUG_FLAGS[:dev_console]
    $stdout = File.new('tmp/output', 'w')
    console = Console::Console.new(lux)
    console_thread = Thread.new { console.run() }
  end

  lux_thread.join

  lux.execute("go;")
  lux.execute("go;")
end

puts "Usage: lux <entry_file>"
