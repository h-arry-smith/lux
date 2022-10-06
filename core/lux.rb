require "listen"

require_relative "ast_printer"
require_relative "interpreter"
require_relative "lexer"
require_relative "parser"
require_relative "cue_engine"
require_relative "lighting_engine"
require_relative "output"
require_relative "timer"
require_relative "world"

module Core
  class Lux
    attr_reader :world

    def initialize(root_directory, debug_flags)
      @root_directory = root_directory
      @debug_flags = debug_flags

      @time = Timer.new()
      @world = make_world

      @lexer = Lexer.new()
      @parser = Parser.new()
      @interpreter = Interpreter.new(self)
      @lighting_engine = LightingEngine.new(@world, @time)
      @cue_engine = CueEngine.new("cues/")

      @output = SACNOutput.new("127.0.0.1")
      @output.connect()

      @listener = watch_files
    end

    def make_world
      world = World.new()
      # Temporary World
      18.times { |x| world.add(Dimmer.new(x + 1, 1, 1 + x)) }
      6.times { |x| world.add(MovingLight.new(x + 101, 1, 101+(6*x))) }

      world
    end

    def start(entry_file)
      puts "Starting lux... #{entry_file}"
      puts "Listening to changes in #{@root_directory}"

      @listener.start
      evaluate_file(entry_file)
      run
    end

    def run()
      while true
        @time.delta_start
        
        @lighting_engine.run()

        if @debug_flags[:dump_universe]
          @lighting_engine.dump_universes
        end

        if @debug_flags[:broadcast]
          @output.broadcast(@lighting_engine.universes)
        end

        delay = @time.target_hz(20)
        sleep(delay) if delay.positive?
      end
    end

    def evaluate_file(file)
      input = File.read(file)
      evaluate(input)
    end

    def evaluate(input)
      @lexer.source = input
      @lexer.scan_tokens

      if @debug_flags[:token]
        puts "# TOKENS #"

        @lexer.tokens.each { |token| puts token }
      end

      @parser.tokens = @lexer.tokens

      begin
        ast = @parser.parse
      rescue ParserError => e
        puts e
        return
      end
        

      if @debug_flags[:ast]
        puts "# AST #"

        ast_printer = AstPrinter.new
        ast_printer.print_ast(ast)
      end

      @interpreter.interpret(ast)

      if @debug_flags[:lx_state]
        puts "# LIGHTING STATE #"
        @interpreter.world.fixtures.each { |fixture| fixture.debug() }
      end

      @world = @interpreter.world
    end

    def reload(file)
      @world.reset
      evaluate_file(file)
    end

    def command(symbol, data = nil)
      case symbol
      when :load
        @cue_engine.load(data[:identifier])
        evaluate_file(@cue_engine.current.cue)
      when :go
        @cue_engine.current.go
        evaluate_file(@cue_engine.current.cue)
      when :goto
        rebuild, files_to_run = @cue_engine.current.goto(data[:cue])
        if rebuild
          rebuild_world_from_files(files_to_run)
        else
          run_files(files_to_run)
        end
      end
    end

    def loaded_cuelist
      @cue_engine.current
    end

    private

    def watch_files
      Listen.to(@root_directory, only: /\.lux$/) do |modified, added, removed|
        start = Time.now.to_f

        modified.each { |file| modified_file(file) }
        added.each { |file| added_file(file) }
        removed.each { |file| removed_file(file) }

        finish = Time.now.to_f

        ms = ((finish-start)*1000).round
        puts "Reloading lighting state... #{ms}ms"
      end
    end

    # TODO : If file is current cue, can we just run from the previous state of the
    # last cue, if it exists?
    def modified_file(file)
      puts "Detected change in: #{file}"
      files_to_rerun = @cue_engine.files_to_rerun(file)
      rebuild_world_from_files(files_to_rerun)
    end

    def rebuild_world_from_files(files_to_rerun)
      @world.reset unless files_to_rerun.empty?
      run_files(files_to_rerun)
    end

    def run_files(files_to_run)
      unless files_to_run.empty?
        files_to_run.each { |file| evaluate_file(file) }
      end
    end

    def added_file(file)
      puts "Added file: #{file}"
    end

    def removed_file(file)
      puts "Removed file: #{file}"
    end
  end
end
