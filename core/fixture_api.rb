require_relative "color_space"
require_relative "parameter"

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

      def param(parameter, default: 0, offset: nil, min: 0, max: 100, fine: false)
        @max_offset = 0 if @max_offset.nil?
        @current_offset = 0 if @current_offset.nil?
        @params = {} if @params.nil?

        @current_offset = offset - 1 unless offset.nil?

        @max_offset = @current_offset if @current_offset > @max_offset

        if @current_group
          @current_group[parameter.to_s] = Parameter.new(parameter.to_s, default, @current_offset, min, max, fine)
        else
          @params[parameter.to_s] = Parameter.new(parameter.to_s, default, @current_offset, min, max, fine)
        end

        if fine
          @current_offset += 2
        else
          @current_offset += 1
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
        @current_offset = opts[:offset] - 1 if opts[:offset]
        
        group :color do
          ColorSpace.value(color_space).each_with_index do |color, index|
            param(color)
          end
        end
      end
    end
  end
end