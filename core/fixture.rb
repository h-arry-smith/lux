require_relative "value"
require_relative "fade"
require_relative "delay"
require_relative "parameter"
require_relative "color_space"

module Core
  module FixtureApi
    def self.included(base)
      base.extend ApiMethods
      base.include FixtureMethods
    end

    module FixtureMethods
      def fixture_params
        self.class.instance_variable_get(:@params)
      end

      def fixture_color_space
        self.class.instance_variable_get(:@color_space)
      end

      def fixture_footprint
        self.class.instance_variable_get(:@max_offset)
      end

      def fixture_name
        self.class.instance_variable_get(:@name)
      end
    end

    module ApiMethods
      def name(name)
        @name = name
      end

      def param(parameter, default: 0, offset: nil, min: 0, max: 100)
        @max_offset = 0 if @max_offset.nil?
        @current_offset = 0 if @current_offset.nil?
        @params = {} if @params.nil?

        if offset.nil?
          offset = @current_offset
          @current_offset += 1
        end

        @max_offset = offset if offset > @max_offset

        if @current_group
          @current_group[parameter.to_s] = Parameter.new(parameter.to_s, default, offset, min, max)
        else
          @params[parameter.to_s] = Parameter.new(parameter.to_s, default, offset, min, max)
        end
      end

      def group(group_parameter)
        @params = {} if @params.nil?
        @current_offset = 0 if @current_offset.nil?

        @params[group_parameter.to_s] = GroupParameter.new(group_parameter.to_s, @current_offset)
        @current_group = @params[group_parameter.to_s]
        yield
        @current_group = nil
      end

      def color(color_space, **opts)
        @color_space = color_space
        group :color do
          ColorSpace.value(color_space).each_with_index do |color, index|
            if opts[:offset]
              opts[:offset] = opts[:offset] + index
            end
            param(color, **opts)
          end
        end
      end
    end
  end

  class Fixture
    include FixtureApi

    attr_reader :id, :params, :universe, :address

    def initialize(id, universe, address)
      @id = id
      @universe = universe
      @address = address
      @params = {}

      initialize_parameters
    end
    
    def apply(identifier, value, time_context)
      parameter = get_parameter(identifier)

      parameter.apply(value, time_context)
    end

    def run(time)
      data = Array.new(fixture_footprint, 0)

      @params.values.each do |parameter|
        dmx_value = parameter.run(time)

        if dmx_value.is_a?(Numeric)
          data[parameter.offset] = dmx_value
        elsif dmx_value.is_a?(Array)
          data[parameter.offset..(dmx_value.length-1)] = dmx_value
        end
      end

      data
    end

    def reset
      @params.values.each { |parameter| parameter.reset }
    end

    def resolve(elapsed_time)
      @params.each { |id, parameter| parameter.resolve(elapsed_time) }
    end

    def initialize_parameters
      fixture_params.each do |symbol, parameter|
        @params[symbol] = parameter.instantiate()
      end
    end

    def debug
      puts "Fixture #{id} - #{fixture_name}"
      @params.values.each do |instance|
        puts "  #{instance.parameter.id}: #{instance.value}"
      end
    end

    private

    def apply_group(group, values, time_context)
      return if values.nil?
      raise RuntimeError.new("Expected a hash to apply to a group") unless values.is_a?(Hash)

      if group.id == "color" && named_tuple?(values)
        values = ColorSpace.convert(values, fixture_color_space)
      end

      if named_tuple_with_correct_arity?(values, group)
        return values.each do |parameter, value|
          instance = get_parameter(parameter.to_s)
          apply_parameter(instance, value&.get(), time_context)
        end
      elsif anonymous_tuple_with_correct_arity?(values, group)
        return group.children.each_with_index do |parameter, index|
          instance = get_parameter(parameter.to_s)
          apply_parameter(instance, values[:"_#{index}"]&.get(), time_context)
        end
      end

      raise RuntimeError.new("Provided tuple has the wrong keys / values")
    end

    def named_tuple?(values)
      values.all? { |k, _| !k.to_s.start_with?("_") }
    end

    def named_tuple_with_correct_arity?(values, group)
      values.keys == group.children
    end

    def anonymous_tuple_with_correct_arity?(values, group)
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

      if @params.key?(parameter)
        return @params[parameter]
      elsif fixture_groups.key?(parameter)
        return fixture_groups[parameter]
      end

      raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{id}")
    end
  end

  class Dimmer < Fixture
    name "Dimmer"
    param :intensity
  end

  class MovingLight < Fixture
    name "Moving Light"
    param :intensity
    group :position do
      param :pan, min: -270, max: 270
      param :tilt, min: -123, max: 123
    end

    color :rgb
  end
end
