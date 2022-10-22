require_relative "value"
require_relative "dynamic_value"
require_relative "fx"

module Core
  class FunctionRegistry
    def initialize
      @functions = {}
    end

    def add(identifier, function)
      @functions[identifier] = function
    end

    def get(identifier)
      @functions[identifier]
    end
  end


  FunctionRegister = FunctionRegistry.new()

  FunctionRegister.add("rgb", ->(_, r, g, b) { return ValueTuple.new({ red: r, green: g, blue: b }) })
  FunctionRegister.add("cmy", ->(_, c, m, y) { return ValueTuple.new({ cyan: c, magenta: m, yellow: y }) })

  FunctionRegister.add("sin", ->(fc, *args) { return DynamicValue.new(FX.method(:sin), fc, args) })
  FunctionRegister.add("cos", ->(fc, *args) { return DynamicValue.new(FX.method(:cos), fc, args) })
  FunctionRegister.add("square", ->(fc, *args) { return DynamicValue.new(FX.method(:square), fc, args) })
  FunctionRegister.add("pulse", ->(fc, *args) { return DynamicValue.new(FX.method(:pulse), fc, args) })
  FunctionRegister.add("cycle", ->(fc, *args) { return DynamicValue.new(FX.method(:cycle), fc, args) })
  FunctionRegister.add("chase", ->(fc, *args) { FX.chase(fc, *args) })
end
