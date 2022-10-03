require_relative "core/lux"

DEBUG_FLAGS = {
  token: false,
  ast: false,
  lx_state: false,
  dump_universe: false,
  broadcast: true
}

if ARGV.length == 1
  entry_file = ARGV[0]
  working_directory = File.dirname(File.expand_path(entry_file))

  lux = Lux.new(working_directory, DEBUG_FLAGS)
  lux.start(entry_file)
end

puts "Usage: lux <entry_file>"
