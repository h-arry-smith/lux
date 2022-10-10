require "curses"
require_relative "colors"
require_relative "tab"
require_relative "cue_list"
require_relative "main_console"

module Console
  class Console < Widget
    def initialize(lux)
      @lux = lux

      @tablist = TabList.new()
      @tablist << Tab.new("console", MainConsole.new(@lux))
      @tablist << Tab.new("cue list", CueList.new(@lux.cue_engine))

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

          @main.setpos(0, 0)

          @tablist.draw(@main)
          @tablist.current_tab.draw(@main)

          clear_rest(@main)

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

    def initialize_curses
      Curses.init_screen
      Curses.start_color
      Curses.curs_set(0)
      Curses.noecho
      Curses.init_pair(BLACK_ON_WHITE, 0, 7)
      Curses.init_pair(YELLOW_ON_BLACK, 3, 0)
      Curses.init_pair(YELLOW_ON_BLUE, 3, 4)
    end

    def handle_key(key)
      case key
      when ' '
        @lux.evaluate("go;")
      when '9'
        @tablist.advance
      end
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


    def clear_rest(win)
      (win.maxy - win.cury).times {win.deleteln()}
    end
  end
end
