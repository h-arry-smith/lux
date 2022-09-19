class SelectionEngine
  def select(world, selector)
    fixture = world.fixture(selector)

    World.new(fixture, world)
  end
end
