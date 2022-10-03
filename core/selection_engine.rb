require_relative "world"

module Core
  class SelectionEngine
    def select(world, query)
      new_world = World.new([], world)

      query.each do |q|
        case q[:type]
        when :single
          new_world.add(single(world, q[:id]))
        when :range
          new_world.add(range(world, q[:start], q[:end]))
        end
      end

      new_world
    end
    private

    def single(world, id)
      world.fixtures.filter { |fixture| fixture.id == id }
    end

    def range(world, first, last)
      world.fixtures.filter { |fixture| fixture.id >= first && fixture.id <= last }
    end
  end
end
