class Fixture
  attr_reader :id

  @@params = []

  def initialize(id)
    @id = id
  end
  
  def apply(parameter, value)
    instance_variable_set("@#{parameter}", value)
  end

  def to_s
    "##{@id} #{debug_params}"
  end

  private
  
  def self.param(parameter)
    attr_accessor parameter
    @@params << parameter
  end

  def debug_params
    @@params.map { |param| "#{param}:#{instance_variable_get("@#{param}")}"}.join(" ")
  end
end

class Dimmer < Fixture
  param :intensity
end
