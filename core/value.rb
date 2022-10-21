module Core
  class Value
    attr_writer :parameter

    def get
      raise NotImplementedError
    end

    def run(_time)
      raise NotImplementedError
    end

    def to_s
      "Value"
    end
  end

  class StaticValue < Value
    attr_reader :value

    def initialize(value)
      @value = value
    end

    def get
      self
    end

    def run(_time)
      @value
    end

    def >(other)
      @value > other.value
    end

    def <(other)
      @value < other.value
    end

    def to_s
      "Static(#{@value})"
    end
  end
  
  class PercentValue < StaticValue
    def to_s
      "Percent(#{@value}%)"
    end
    
    def run(_time)
      return @value if @parameter.nil?
      
      factor = @value / 100.0
      difference = @parameter.max - @parameter.min
      
      @parameter.min + (difference * factor)
    end
  end

  class ValueTuple < Value
    def initialize(values)
      @values = values
    end

    def value
      @values
    end

    def get
      ValueTuple.new(@values.transform_values { |v| v&.get })
    end

    def run(time, _parameter = nil)
      @values 
    end

    def to_s
      values = @values.values.map(&:to_s).join(" ")
      "Tuple{ #{values} }"
    end

    def name_as(tuple)
      index = 0
      @values.transform_keys! do
        key = tuple.value.keys[index]
        index += 1
        key
      end
      self
    end

    def named_tuple?
      @values.all? { |k, _| !k.to_s.start_with?("_") }
    end

    def anonymous_tuple?
      @values.all? { |k, _| k.to_s.start_with?("_") }
    end
  end
end
