require_relative "value"
require_relative "fade"
require_relative "delay"
require_relative "color_space"
require_relative "fixture_api"

module Core
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
          dmx_value.flatten!
          start = parameter.offset
          finish = start + dmx_value.length
          data[start...finish] = dmx_value
        end
      end

      data
    end

    def reset
      @params.values.each { |parameter| parameter.reset }
      p @params
    end

    def resolve(elapsed_time)
      @params.each { |id, parameter| parameter.resolve(elapsed_time) }
    end

    def fast_forward
      @params.each { |id, parameter| parameter.fast_foward }
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
      end

      raise RuntimeError.new("Parameter #{parameter} not valid for fixture #{id}")
    end
  end
end