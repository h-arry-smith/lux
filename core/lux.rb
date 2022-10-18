require "listen"
require "pathname"

require_relative "ast_printer"
require_relative "interpreter"
require_relative "lexer"
require_relative "parser"
require_relative "cue_engine"
require_relative "lighting_engine"
require_relative "output"
require_relative "time_context"
require_relative "timer"
require_relative "world"
require_relative "fixture_library"

module Core
  class Lux
    attr_reader :world, :lighting_engine, :time, :cue_engine

    def initialize(root_directory, debug_flags)
      @root_directory = root_directory
      @debug_flags = debug_flags

      @fixture_library = FixtureLibrary.new('fixture_library')

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
      dimmer = @fixture_library["generic/dimmer"]
      moving_light = @fixture_library["generic/moving-light"]
      robe_1200 = @fixture_library["robe/colorwash_1200E_AT-m1"]
      
      18.times { |x| world.add(dimmer.new(x + 1, 1, 1 + x)) }
      6.times { |x| world.add(moving_light.new(x + 101, 1, 101+(6*x))) }
      world.add(robe_1200.new(201, 1, 301))

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
        evaluate_file(@cue_engine.current.cue.path)
        reset_time
      when :go
        @cue_engine.current.go
        @world.resolve_fades(@time.elapsed)

        puts "#{"#"*20}    CUE #{@cue_engine.current.cue}    #{"#"*20}"

        evaluate_file(@cue_engine.current.cue.path)
        reset_time
      when :goto
        rebuild, files_to_run = @cue_engine.current.goto(data[:cue])
        if rebuild
          rebuild_world_from_files(files_to_run)
        else
          run_files(files_to_run)
        end
        reset_time
      end
    end

    def reset_time
      @time.start
      @world.reset_time
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

    # TODO: If file is current cue, can we just run from the previous state of the
    # last cue, if it exists?

    # TODO: Detect what a file is, if it isn't a cue, then is it something we
    # need to reevaluate?

    def modified_file(file)
      puts "Detected change in: #{file}"
      rebuild_from_file(file)
    end


    def added_file(file)
      puts "Added file: #{file}"
      path = Pathname.new(file)
      cuelist_identifier = path.dirname.basename.to_s

      @cue_engine.reload(cuelist_identifier)

      rebuild_from_file(file)
    end

    def removed_file(file)
      puts "Removed file: #{file}"
      path = Pathname.new(file)
      cuelist_identifier = path.dirname.basename.to_s
      @cue_engine.reload(cuelist_identifier)

      rebuild_from_file(file)
    end

    def rebuild_from_file(file)
      files_to_rerun = @cue_engine.files_to_rerun(file)

      puts "REBUILDING FILES"
      p files_to_rerun

      if only_file_is_current_cue?(files_to_rerun)
        rebuild_world_from_files(files_to_rerun)
      else
        rebuild_world_from_files(files_to_rerun)
        reset_time
      end
    end

    def only_file_is_current_cue?(files_to_rerun)
      return false if files_to_rerun.length > 1
      file = files_to_rerun.first

      @cue_engine.current.cue == file
    end

    def rebuild_world_from_files(files_to_rerun)
      unless files_to_rerun.empty?
        @world.reset
        run_files(files_to_rerun)
      end
    end

    def run_files(files_to_run)
      unless files_to_run.empty?
        fast_foward_cues = files_to_run[...-1]
        actual_cue = files_to_run.last

        fast_foward_cues.each do |file|
          evaluate_file(file)
          @world.fast_foward
        end

        evaluate_file(actual_cue)
      end
    end

  end
end
