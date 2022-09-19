class World
  attr_reader :fixtures
  
  def initialize(fixtures = [], parent = nil)
    @parent = parent
    @fixtures = fixtures
  end

  # TODO : Any selection never has duplicates..
  def add(fixture)
    if fixture.is_a?(Array)
      @fixtures.concat(fixture)
    else
      @fixtures << fixture
    end
  end

  def deselect
    return @parent
  end
end
