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

  FunctionRegister.add("rgb", ->(r, g, b) { return ValueTuple.new({ red: r, green: g, blue: b }) })
  FunctionRegister.add("cmy", ->(c, m, y) { return ValueTuple.new({ cyan: c, magenta: m, yellow: y }) })

  FunctionRegister.add("sin", ->(*args) { return DynamicValue.new(FX.method(:sin), args) })

  FunctionRegister.add("cycle", ->(*args) { return DynamicValue.new(FX.method(:cycle), args) })
end
