require "curses"
require_relative "widget"

module Console
  class Fixture
    def initialize(fixture)
      @fixture = fixture
    end

    def length
      id = format("%4d", @fixture.id)
      dmx_values = Array.new(@fixture.fixture_footprint + 1, 0)
      dmx_string = dmx_values.map { |v| format("%03d", v) }.join(" ")

      "#{id} | #{dmx_string}".length
    end

    def draw(lux, window)
      return if lux.lighting_engine.universes.empty?

      id = format("%4d", @fixture.id)
      universe = lux.lighting_engine.universes[@fixture.universe - 1]
      dmx_values = universe.get(@fixture.address, @fixture.fixture_footprint)
      dmx_string = dmx_values.map { |v| format("%03d", v) }.join(" ")

      window.attron(Curses.color_pair(YELLOW_ON_BLACK))
      window << " #{id} | "
      window.attroff(Curses.color_pair(YELLOW_ON_BLACK))

      window << dmx_string
    end
  end
end
