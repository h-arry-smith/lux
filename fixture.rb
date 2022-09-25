require_relative "value"
require_relative "fade"
require_relative "delay"
require_relative "parameter"

module FixtureApi
  def self.included(base)
    base.extend ApiMethods
  end

  module ApiMethods
    def param(parameter)
      @params = {} if @params.nil?

      @params[parameter.to_s] = Parameter.new(parameter.to_s, 0)
      @current_group&.add_child_parameter(parameter)
    end

    def group(group_parameter)
      @groups = {} if @groups.nil?

      @groups[group_parameter.to_s] = GroupParameter.new(group_parameter.to_s)
      @current_group = @groups[group_parameter.to_s]
      yield
      @current_group = nil
    end
  end
end

class Fixture
  include FixtureApi

  attr_reader :id, :params

  def initialize(id)
    @id = id
    @params = {}

    initialize_parameters
  end
  
  def apply(parameter, value, time_context)
    current = get_parameter(parameter)

    value = Fade.from(current, value, time_context)
    value = Delay.from(current, value, time_context)
    
    set_parameter(parameter, value)
  end

  def to_s
    "##{@id} #{debug_params}"
  end

  def initialize_parameters
    params = self.class.instance_variable_get(:@params)
    params.each do |symbol, parameter|
      @params[symbol] = ParameterInstance.new(parameter)
    end
  end

  private

  def set_parameter(parameter, value)
    raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{id}") unless @params.key?(parameter)

    @params[parameter].value = value
  end

  def get_parameter(parameter)
    raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{id}") unless @params.key?(parameter)

    @params[parameter].value
  end

  def debug_params
    @params.map { |param, _| "#{param}:#{get_parameter(param)}"}.join(" ")
  end
end

class Dimmer < Fixture
  param :intensity
end

class MovingLight < Fixture
  param :intensity
  group :position do
    param :pan
    param :tilt
  end
end
