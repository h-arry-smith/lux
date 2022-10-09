require "curses"
require_relative "colors"

module Console
  class Console
    def initialize(lux)
      @lux = lux
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
          draw_console

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
      end
    end

    def initialize_curses
      Curses.init_screen
      Curses.start_color
      Curses.curs_set(0)
      Curses.noecho
      Curses.init_pair(BLACK_ON_WHITE, 0, 7)
      Curses.init_pair(YELLOW_ON_BLACK, 3, 0)
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

    def draw_console
      @main.setpos(0, 1)
      @main << @lux.loaded_cuelist.to_s
      @main.clrtoeol
      @main << "\n"

      @lux.world.fixtures.each { |fixture| draw_fixture(fixture) }
      @main << "\n"
      @main << "\n"
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
      @main.clrtoeol
      @main << "\n"
    end

    def clear_rest
      (@win.maxy - @win.cury - 2).times {@win.deleteln()}
    end
  end
end
