require "curses"
require_relative "colors"

module Console
  class Console
    def initialize(lux)
      @lux = lux
      @tabs = ["console", "cue list"]
      @current_tab = 0
      initialize_curses
    end

    def run
      begin
        @win = create_window
        @main = create_main_window

        @win.timeout = 0

        loop do
          @win.setpos(0, 0)
          draw_border
          draw_tabs

          case @tabs[@current_tab]
          when "console"
            draw_console
          when "cue list"
            draw_cue_list
          end

          key = @win.getch.to_s
          @win.setpos(0, 40)
          @win << key

          handle_key(key)

          @win.refresh
          @main.refresh

          sleep(1 / 30.0)
        end
      ensure
        Curses.close_screen
      end
    end

    private

    def handle_key(key)
      case key
      when ' '
        @lux.evaluate("go;")
      when '9'
        advance_tab
      end
    end

    def advance_tab
      @current_tab += 1
      @current_tab = 0 if @current_tab >= @tabs.length
    end

    def initialize_curses
      Curses.init_screen
      Curses.start_color
      Curses.curs_set(0)
      Curses.noecho
      Curses.init_pair(BLACK_ON_WHITE, 0, 7)
      Curses.init_pair(YELLOW_ON_BLACK, 3, 0)
      Curses.init_pair(YELLOW_ON_BLUE, 3, 4)
    end

    def create_window
      Curses::Window.new(0, 0, 0, 0)
    end

    def create_main_window
      @win.subwin(@win.maxy-1, @win.maxx-2, 1, 1)
    end

    def draw_border
      @win.attron(Curses.color_pair(1))
      @win << " " * @win.maxx

      @win.setpos(@win.maxy-1, 0)
      @win << " " * @win.maxx

      @win.maxx.times do |i|
        @win.setpos(i, 0)
        @win.addch(" ")
        @win.setpos(i, @win.maxx-1)
        @win.addch(" ")
      end

      @win.setpos(0, 1)
      @win << "Lux Dev Console | #{time}"

      @win.attroff(Curses.color_pair(1))
    end

    def time
      raw = @lux.time.elapsed

      hours = raw / (60 * 60)
      minutes = raw / 60
      seconds = raw % 60

      format("%02d:%02d:%02d", hours, minutes, seconds)
    end

    def draw_tabs
      @main.setpos(0, 0)
      @tabs.each_with_index do |tab, index|
        tab_string = "   #{tab}   "
        if @current_tab == index
          @main.attron(Curses.color_pair(YELLOW_ON_BLUE)) { @main << tab_string }
        else
          @main << tab_string
        end
      end
      @main.clrtoeol
      @main << "\n"
    end

    def draw_cue_list
      @main.setpos(1, 0)

      @main.attron(Curses.color_pair(BLACK_ON_WHITE))
      @main << " "
      @main << @lux.cue_engine.current.to_s
      space_to_end(@main)
      @main.attroff(Curses.color_pair(BLACK_ON_WHITE))

      @lux.cue_engine.current.cues.each_with_index do |cue, index|
        if cue == @lux.cue_engine.current.cue
          @main.attron(Curses.color_pair(YELLOW_ON_BLUE))
          draw_cue(cue, index)
          @main.attroff(Curses.color_pair(YELLOW_ON_BLUE))
        else
          draw_cue(cue, index)
        end
      end

      clear_rest(@main)
    end

    def draw_cue(cue, index)
      @main << format(" %4d", index)
      @main << " | "

      if cue.metadata["title"]
        @main << cue.metadata["title"]
      else
        @main << cue.name
      end

      space_to_end(@main)
    end

    def space_to_end(win)
      spaces = win.maxx - win.curx
      win << " " * spaces
    end

    def draw_console
      @main.setpos(1, 0)
      @main << " "
      @main << @lux.loaded_cuelist.to_s
      @main.clrtoeol
      @main << "\n"

      draw_fixtures
      clear_rest(@main)
    end

    def draw_fixtures
      grouped_fixtures = @lux.world.fixtures.group_by { |f| f.fixture_name }

      grouped_fixtures.each do |name, fixtures|
        slice_size = (@main.maxx - 10) / fixture_string_length(fixtures.first)
        @main.attron(Curses::A_BOLD) { @main << " #{name} - #{fixtures.length} - #{slice_size}\n" }
        fixtures.each_slice(slice_size) do |row|
          row.each { |f| draw_fixture(f) }
          @main.clrtoeol
          @main << "\n"
        end
      end
    end

    def fixture_string_length(fixture)
      id = format("%4d", fixture.id)
      dmx_values = Array.new(fixture.fixture_footprint + 1, 0)
      dmx_string = dmx_values.map { |v| format("%03d", v) }.join(" ")

      "#{id} | #{dmx_string}".length
    end

    def draw_fixture(fixture)
      return if @lux.lighting_engine.universes.empty?

      id = format("%4d", fixture.id)
      universe = @lux.lighting_engine.universes[fixture.universe - 1]
      dmx_values = universe.get(fixture.address, fixture.fixture_footprint)
      dmx_string = dmx_values.map { |v| format("%03d", v) }.join(" ")

      @main.attron(Curses.color_pair(YELLOW_ON_BLACK))
      @main << " #{id} | "
      @main.attroff(Curses.color_pair(YELLOW_ON_BLACK))

      @main << dmx_string
    end

    def clear_rest(win)
      (win.maxy - win.cury).times {win.deleteln()}
    end
  end
end
