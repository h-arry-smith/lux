require "curses"
require_relative "widget"

module Console
  class TabList < Widget
    def initialize
      @tabs = []
      @current = 0
    end

    def draw(window)
      @tabs.each do |tab|
        if tab == current_tab
          tab.draw_tab_active(window)
        else
          tab.draw_tab(window)
        end
      end
      window.clrtoeol
      window << "\n"
    end

    def <<(tab)
      @tabs << tab
    end

    def current_tab
      @tabs[@current]
    end

    def advance
      @current += 1
      @current = 0 if @current >= @tabs.length
    end
  end

  class Tab < Widget
    attr_reader :name

    def initialize(name, widget)
      @name = name
      @widget = widget
    end

    def draw(window)
      @widget.draw(window)
    end

    def draw_tab(window)
      window << "   #{@name}   "
    end

    def draw_tab_active(window)
      window.attron(Curses.color_pair(YELLOW_ON_BLUE))
      draw_tab(window)
      window.attroff(Curses.color_pair(YELLOW_ON_BLUE))
    end
  end
end
