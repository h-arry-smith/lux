require_relative "universe"
require_relative "output"

class LightingEngine
  attr_reader :universes
  attr_writer :world

  def initialize(world, timer)
    @timer = timer
    @world = world
    @universes = []
  end

  def run()
    @world.fixtures.each do |fixture|
      universe = get_or_create_universe(fixture.universe)

      data = fixture.run(@timer.elapsed)

      universe.apply(fixture.address, data)
    end
  end

  def dump_universes
    @universes.each { |universe| universe.dump }
  end

  private

  def get_or_create_universe(universe)
    # Universes are 1 indexed, not 0
    universe = universe - 1
    
    if @universes[universe].nil?
      @universes[universe] = Universe.new(universe + 1)
    end

    @universes[universe]
  end
end
