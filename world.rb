class World
  attr_reader :fixtures
  
  def initialize(fixtures = [], parent = nil)
    @parent = parent
    @fixtures = fixtures
  end

  def add_fixture(fixture)
    @fixtures << fixture
  end

  def fixture(number)
    @fixtures.filter { |fixture| fixture.id == number }
  end

  def deselect
    return @parent
  end
end
