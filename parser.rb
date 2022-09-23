require_relative "ast"
require_relative "token"

class Parser
  def initialize(tokens)
    @tokens = tokens
    @current = 0
  end

  def parse
    program
  end

  private

  def program
    statements = []

    statements << statement while !at_end?

    statements
  end

  def expression_statement
    return statement if !check(Token::IDENTIFIER)
    return expression
  end

  def expression
    apply
  end

  def statement
    return selection if check(Token::LEFT_BRACKET)
    return timer if check(Token::AT)
  end

  def timer
    times = []

    until check(Token::LEFT_BRACE)
      times << time
    end

    consume(Token::LEFT_BRACE, "Expect '{' after time clause")

    return Ast::TimeBlock.new(times, block)
  end

  def time
    consume(Token::AT, "Time clause must start with '@'")

    keyword = time_keyword

    consume(Token::NUMBER, "Expected number after time directive")
    value = previous.literal
    consume(Token::SECONDS, "Expected seconds 's' after number")

    return Ast::Time.new(keyword, value)
  end

  def time_keyword
    if match(*TIME_KEYWORDS)
      return previous.lexeme
    end

    raise ParserError.new(peek, "Invalid time keyword")
  end

  def block
    statements = []

    while !check(Token::RIGHT_BRACE) && !at_end?
      statements << expression_statement
    end
    
    consume(Token::RIGHT_BRACE, "Expect '}' after block")

    return Ast::Block.new(statements)
  end

  def apply
    param = parameter

    val = arguments

    consume(Token::SEMICOLON, "Expected semicolon after apply")

    return Ast::Apply.new(param, val)
  end

  def parameter
    id = identifier
    consume(Token::COLON, "Expect ':' after parameter")
    id
  end

  def identifier
    if match(Token::IDENTIFIER)
      return previous.literal
    end

    error(peek, "Expected valid identifer")
  end

  def arguments
    args = []

    until check(Token::SEMICOLON)
      val = value

      if check(Token::ARROW)
        val = range
      end

      args << val
    end

    args
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

    if check(Token::NUMBER)
      first = value

      select = Ast::Selector.new(first)

      if check(Token::ARROW)
        select =  Ast::Selector.new(range)
      end

      consume(Token::RIGHT_BRACKET, "Expect ']' to end selector")
      return select
    end

    error(peek, "Expected valid selector")
  end

  def range
    first = previous.literal

    consume(Token::ARROW, "Expect -> for range.")

    if match(Token::NUMBER)
      return Ast::Range.new(first, previous.literal)
    end

    error(peek, "Expected a end number for range.")
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
    return false if at_end?
    peek.type == type
  end

  def advance
    @current += 1 unless at_end?
    previous
  end

  def at_end?
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
