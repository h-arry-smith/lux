require_relative "value"
require_relative "fade"

class Fixture
  attr_reader :id

  @@params = []

  def initialize(id)
    @id = id

    set_defaults
  end
  
  def apply(parameter, value, time_context)
    current = get_parameter(parameter)

    value = Fade.from(current, value, time_context)
    
    set_parameter(parameter, value)
  end

  def to_s
    "##{@id} #{debug_params}"
  end

  private

  def set_parameter(parameter, value)
    raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{self.id}") unless instance_variable_defined?("@#{parameter}")

    instance_variable_set("@#{parameter}", value)
  end

  def get_parameter(parameter)
    raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{self.id}") unless instance_variable_defined?("@#{parameter}")

    instance_variable_get("@#{parameter}")
  end
  
  def self.param(parameter)
    attr_accessor parameter
    @@params << parameter
  end

  def debug_params
    @@params.map { |param| "#{param}:#{get_parameter(param)}"}.join(" ")
  end

  # Temporary, in the future defaults should be set from the param dsl :)
  def set_defaults
    @@params.each { |param| instance_variable_set("@#{param}", StaticValue.new(0)) }
  end
end

class Dimmer < Fixture
  param :intensity
end
