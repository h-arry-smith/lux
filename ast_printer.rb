class AstPrinter
  def initialize
    @indent = 0
  end

  def print_ast(ast)
    ast.each { |node| print_tree(node) }
  end

  def print_tree(node)
    @indent = 0
    self.print(node)
  end

  def visit_apply(node)
    print_with_indent("APPLY")
    indent
    print_with_indent("PARAMETER #{node.parameter}")
    indent
    node.value.each { |node| self.print(node) }
    dedent
    dedent
  end

  def visit_block(node)
    print_with_indent("BLOCK")
    indent
    node.statements.each { |statement| self.print(statement) }
    dedent
  end

  def visit_selector(node)
    print_with_indent("SELECTOR")
    indent
    self.print(node.selector)
    dedent
  end

  def visit_selection(node)
    print_with_indent("SELECTION")
    indent
    self.print(node.selector)
    self.print(node.block)
    dedent
  end

  def visit_tuple(node)
    print_with_indent("TUPLE {")
    indent
    node.values.each do |key, value|
      print_with_indent("#{key} :")
      self.print(value)
    end
    dedent
    print_with_indent("}")
  end

  def visit_call(node)
    print_with_indent("CALL #{node.identifier}")
    indent
    node.arguments.each_with_index do |arg, index|
      print_with_indent "ARG #{index}"
      self.print(arg)
    end
    dedent
  end

  def visit_value(node)
    print_with_indent("VALUE #{node.value}")
  end

  def visit_range(node)
    print_with_indent("RANGE #{node.start}->#{node.end}")
  end

  def visit_timeblock(node)
    print_with_indent("TIME BLOCK")
    indent
    node.time.each { |time| self.print(time) }
    self.print(node.block)
    dedent
  end

  def visit_time(node)
    print_with_indent("TIME #{node.keyword} #{node.value}")
  end

  private

  def print(node)
    node.accept(self)
  end

  def print_with_indent(text)
    puts "#{" "*@indent*2}#{text}"
  end

  def indent
    @indent += 1
  end

  def dedent
    @indent -= 1
  end
end
