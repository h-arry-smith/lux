AST = {
  Apply: [:parameter, :value],
  Block: [:statements],
  Selector: [:selector],
  Selection: [:selector, :block],
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
