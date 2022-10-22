require_relative "value"
require_relative "dynamic_value"

module Core
  class GroupParameterInstance
    attr_accessor :parameter, :value

    def initialize(parameter)
      @parameter = parameter
      instantiate_children
      @value = default_tuple_from_children
      @time_context = nil
    end
    
    def id
      @parameter.id
    end

    def reset
      @value = default_tuple_from_children
    end

    def resolve(time)
      if @value.is_a?(Fade) || @value.is_a?(Delay)
        @value = @value.resolve(time)
      end
    end

    def fast_foward
      if @value.is_a?(Fade) || @value.is_a?(Delay)
        @value = @value.fast_foward
      end
    end

    def apply(new_value, time_context)
      return if new_value.nil?

      if new_value.is_a?(DynamicValue)
        new_value.resolve_nil_values(@value)
      end

      new_value = Fade.from(@value, new_value, time_context)
      new_value = Delay.from(@value, new_value, time_context)

      @value = new_value
    end

    def run(time)
      value = @value.run(time)
      return if @value.nil?

      if named_tuple? value
        value.each do |id, value|
          @children[id].value = value
        end
      end

      if anonymous_tuple? value
        index = 0
        @children.each do |_, child|
          new_value = value[:"_#{index}"]
          child.value = new_value unless new_value.nil?
          index += 1
        end
      end

      @children.map { |_, child| child.run(time) }
    end

    def offset
      @parameter.offset
    end

    private

    def named_tuple?(values)
      values.all? { |k, _| !k.to_s.start_with?("_") }
    end

    def anonymous_tuple?(values)
      values.all? { |k, _| k.to_s.start_with?("_") }
    end

    def instantiate_children
      @children = {}
      @parameter.children.each { |id, parameter| @children[id.to_sym] = ParameterInstance.new(parameter) }
    end

    def default_tuple_from_children
      defaults = {}
      @children.each { |id, child| defaults[id.to_sym] = child.parameter.default }
      ValueTuple.new(defaults)
    end
  end

  class ParameterInstance
    attr_accessor :parameter

    def initialize(parameter)
      @parameter = parameter
      @value = parameter.default
    end
    
    def value
      @value
    end
    
    def value=(value)
      @value = value
      value.parameter = @parameter
    end
    
    def id
      @parameter.id
    end

    def reset
      @value = @parameter.default
    end

    def resolve(time)
      if @value.is_a?(Fade) || @value.is_a?(Delay)
        @value = @value.resolve(time)
      end
    end

    def fast_foward
      if @value.is_a?(Fade) || @value.is_a?(Delay)
        @value = @value.fast_foward
      end
    end

    def apply(new_value, time_context)
      return if new_value.nil?

      new_value.resolve_nil_values(@value) if new_value.is_a?(DynamicValue)

      new_value = Fade.from(@value, new_value, time_context)
      new_value = Delay.from(@value, new_value, time_context)

      @value = new_value
    end

    def run(time)
      current_value = @value.run(time)
      @parameter.to_dmx(current_value)
    end

    def offset
      @parameter.offset
    end
  end

  class Parameter
    attr_reader :id, :default, :offset, :min, :max

    def initialize(id, default, offset, min, max, fine)
      @id = id
      @default = StaticValue.new(default)
      @offset = offset
      @min = min
      @max = max
      @fine = fine
    end

    def instantiate
      ParameterInstance.new(self)
    end

    def to_dmx(current_value)
      return to_dmx_fine(current_value) if @fine
      return 255 if current_value >= @max
      return 0 if current_value <= @min

      (factor(current_value) * 255).round
    end
    
    def to_dmx_fine(current_value)
      absolute_value = difference * factor(current_value)
      step = difference / (256.0 * 256.0)
      steps = absolute_value / step
      
      main_part = (steps / 256).round
      fractional_part = (steps % 256).round
      
      [main_part, fractional_part]
    end
    
    def factor(current_value)
      relative_value = current_value - @min
      relative_value.to_f / difference.to_f
    end
    
    def difference
      @max - @min
    end

    def to_s
      "#{@parameter}"
    end
  end

  class GroupParameter
    attr_reader :id, :children, :offset

    def initialize(id, offset)
      @id = id
      @children = {}
      @offset = offset
    end

    def instantiate
      GroupParameterInstance.new(self)
    end

    def []=(id, parameter)
      @children[id] = parameter
    end
  end
end
