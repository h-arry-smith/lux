class AstPrinter
  def initialize
    @indent = 0
  end

  def print_tree(node)
    @indent = 0
    self.print(node)
  end

  def visit_apply(node)
    print_with_indent("APPLY parameter:#{node.parameter} value:#{node.value}")
  end

  def visit_block(node)
    print_with_indent("BLOCK statement:#{node.statements}")
  end

  def visit_selector(node)
    print_with_indent("SELECTOR selector:#{node.selector}")
  end

  def visit_selection(node)
    print_with_indent("SELECTION")
    indent
    self.print(node.selector)
    self.print(node.block)
    dedent
  end

  def visit_value(node)
    print_with_indent("VALUE value:#{node.value}")
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
