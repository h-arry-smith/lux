AST = {
  Apply: [:parameter, :value],
  Block: [:statements],
  Call: [:identifier, :arguments],
  Go: [:identifier],
  Goto: [:cue],
  Load: [:identifier],
  Range: [:start, :end],
  Selector: [:selector],
  Selection: [:selector, :block],
  Tuple: [:values],
  Time: [:keyword, :value],
  TimeBlock: [:time, :block],
  Value: [:value],
  VarDefine: [:identifier, :block],
  VarFetch: [:identifier]
}

module Core
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
end

