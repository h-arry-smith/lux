require_relative "universe"

class LightingEngine
  def initialize
    @universes = []
  end

  def run(world)
    world.fixtures.each do |fixture|
      universe = get_or_create_universe(fixture.universe)

      data = fixture.run(0)

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
      @universes[universe] = Universe.new(universe)
    end

    @universes[universe]
  end
end
