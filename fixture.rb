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
  
  def apply(identifier, value, time_context)
    parameter = get_parameter(identifier)

    if parameter.is_a?(ParameterInstance)
      return apply_parameter(parameter, value, time_context)
    elsif parameter.is_a?(GroupParameter)
      return apply_group(parameter, value, time_context)
    end

    raise RuntimeError.new("Unhandled parameter type: #{parameter}")
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

  def apply_parameter(instance, value, time_context)
    current = instance.value

    value = Fade.from(current, value, time_context)
    value = Delay.from(current, value, time_context)

    set_parameter(instance.parameter.id, value)
  end

  def apply_group(group, values, time_context)
    return if values.nil?
    raise RuntimeError.new("Expected a hash to apply to a group") unless values.is_a?(Hash)

    if named_tuple_with_correct_arity(values, group)
      return values.each do |parameter, value|
        instance = get_parameter(parameter.to_s)
        apply_parameter(instance, value&.get(), time_context)
      end
    elsif anonymous_tuple_with_correct_arity(values, group)
      return group.children.each_with_index do |parameter, index|
        instance = get_parameter(parameter.to_s)
        apply_parameter(instance, values[:"_#{index}"]&.get(), time_context)
      end
    end

    raise RuntimeError.new("Provided tuple has the wrong keys / values")
  end

  def named_tuple_with_correct_arity(values, group)
    values.keys == group.children
  end

  def anonymous_tuple_with_correct_arity(values, group)
    unless values.all? { |k, _| k.to_s.start_with?("_") }
      raise RuntimeError.new("Do not mix anonymous tuple with keyed tuple")
    end

    values.keys.length == group.children.length
  end

  def set_parameter(parameter, value)
    raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{id}") unless @params.key?(parameter)

    @params[parameter].value = value
  end

  def get_parameter(parameter)
    groups = self.class.instance_variable_get(:@groups)

    if @params.key?(parameter)
      return @params[parameter]
    elsif groups.key?(parameter)
      return groups[parameter]
    end

    raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{id}")
  end

  def debug_params
    @params.map { |param, _| "#{param}:#{get_parameter(param).value}"}.join(" ")
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
