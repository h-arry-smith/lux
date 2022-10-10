require "curses"
require_relative "widget"

module Console
  class CueList < Widget
    def initialize(cue_engine)
      @cue_engine = cue_engine
    end

    def draw(window)
      draw_header(window)

      @cue_engine.current.cues.each_with_index do |cue, index|
        cue_widget = Cue.new(cue, index)

        if cue == @cue_engine.current.cue
          cue_widget.draw_active(window)
        else
          cue_widget.draw(window)
        end
      end
    end

    def draw_header(window)
      window.attron(Curses.color_pair(BLACK_ON_WHITE))

      window << " "
      window << @cue_engine.current.to_s

      space_to_end(window)

      window.attroff(Curses.color_pair(BLACK_ON_WHITE))
    end
  end

  class Cue < Widget
    def initialize(cue, index)
      @cue = cue
      @index = index
    end

    def draw(window)
      window << format(" %4d", @index)
      window << " | "

      if @cue.metadata["title"]
        window << @cue.metadata["title"]
      else
        window << @cue.name
      end

      space_to_end(window)
    end

    def draw_active(window)
      window.attron(Curses.color_pair(YELLOW_ON_BLUE)) \
        { draw(window) }
    end
  end
end
