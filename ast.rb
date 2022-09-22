AST = {
  Apply: [:parameter, :value],
  Block: [:statements],
  Range: [:start, :end],
  Selector: [:selector],
  Selection: [:selector, :block],
  Time: [:keyword, :value],
  TimeBlock: [:time, :block],
  Value: [:value]
}

module Ast
end

AST.each_pair do |name, params|
  Ast.const_set(
    name,
    Struct.new(*params) do
      define_method(:accept) do |visitor|
        visitor.public_send("visit_#{name.downcase}", self)
      end
    end)
end
