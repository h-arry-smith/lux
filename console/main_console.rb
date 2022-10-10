require "curses"
require_relative "widget"
require_relative "fixture"

module Console
  class MainConsole < Widget
    def initialize(lux)
      @lux = lux
    end

    def draw(window)
      draw_header(window)
      draw_fixtures(window)
    end

    private

    def draw_header(window)
      window << " "
      window << @lux.loaded_cuelist.to_s
      window.clrtoeol
      window << "\n"
    end

    def draw_fixtures(window)
      grouped_fixtures = @lux.world.fixtures.group_by { |f| f.fixture_name }

      grouped_fixtures.each do |name, fixtures|
        draw_fixture_group_header(name, fixtures, window)

        size = slice_size(fixtures.first, window)
        fixtures.each_slice(size) do |row|
          row.each { |f| Fixture.new(f).draw(@lux, window) }
          window.clrtoeol
          window << "\n"
        end
      end
    end

    def draw_fixture_group_header(name, fixtures, window)
      window.attron(Curses::A_BOLD) \
        { window << " #{name} - #{fixtures.length}\n" }
    end

    def slice_size(fixture, window)
      example = Fixture.new(fixture)
      (window.maxx - 10) / example.length
    end
  end
end
