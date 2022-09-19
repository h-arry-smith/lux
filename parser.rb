require_relative "ast"
require_relative "token"

class Parser
  def initialize(tokens)
    @tokens = tokens
    @current = 0
  end

  def parse
    statement
  end

  private

  def statement
    selection
  end

  def block
    statements = []

    while !check(Token::RIGHT_BRACE) && !is_at_end
      statements << apply
    end
    
    consume(Token::RIGHT_BRACE, "Expect '}' after block")

    return Ast::Block.new(statements)
  end

  def apply
    parameter = identifier

    consume(Token::COLON, "Expect ':' after identifier")

    val = value

    consume(Token::SEMICOLON, "Expected semicolon after apply")

    return Ast::Apply.new(parameter, val)
  end

  def identifier
    if match(Token::IDENTIFIER)
      return previous.literal
    end

    error(peek, "Expected valid identifer")
  end

  def value
    if match(Token::NUMBER)
      return Ast::Value.new(previous.literal)
    end

    error(peek, "Expected a valid value")
  end

  def selection
    select = selector

    consume(Token::LEFT_BRACE, "Expect block after selector")
    return Ast::Selection.new(select, block)
  end

  def selector
    consume(Token::LEFT_BRACKET, "Expect '[' to start selector")

    if match(Token::NUMBER)
      select = Ast::Selector.new(previous.literal)
      consume(Token::RIGHT_BRACKET, "Expect ']' to end selector")
      return select
    end

    error(peek, "Expected valid selector")
  end

  def match(*types)
    types.each do |type|
      if check(type)
        advance
        return true
      end
    end

    false
  end

  def consume(type, message)
    return advance if check(type)

    raise error(peek, message)
  end

  def check(type)
    return false if is_at_end
    peek.type == type
  end

  def advance
    @current += 1 unless is_at_end
    previous
  end

  def is_at_end
    peek.type == Token::EOF
  end

  def peek
    @tokens[@current]
  end

  def previous
    @tokens[@current - 1]
  end

  def error(peek, message)
    raise ParserError.new(peek, message)
  end
end

class ParserError < StandardError
  def initialize(peek, message)
    super("Parser Error: [#{peek}] #{message}")
  end
end
