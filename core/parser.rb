require_relative "ast"
require_relative "token"

module Core
  class Parser
    def initialize
      @tokens = nil
      @current = 0
    end

    def tokens=(tokens)
      @tokens = tokens
      @current = 0
    end

    def parse
      raise RuntimeError, "No tokens to parse" if @tokens.nil?

      program
    end

    private

    def program
      statements = []

      statements << statement while !at_end?

      statements
    end

    def expression_statement
      return variable if match(Token::HASH)
      return statement unless check(Token::IDENTIFIER)
      apply
    end

    def statement
      return command if match(*COMMAND_KEYWORDS)
      return variable if match(Token::HASH)
      return block if match(Token::LEFT_BRACE)
      return selection if check(Token::LEFT_BRACKET)
      return timer if check(Token::AT)

      error(peek, "Unexpected token.")
    end

    def command
      command = nil

      case previous.type
      when Token::LOAD
        if match(Token::IDENTIFIER)
          command = Ast::Load.new(previous)
        else
          peek(error, "Expected a cue list identifier after load command")
        end
      when Token::GO
        command = Ast::Go.new(nil)
      when Token::GOTO
        cue = value
        command = Ast::Goto.new(cue.value)
      end

      consume(Token::SEMICOLON, "Commands must end with a semicolon.")
      command
    end

    def variable
      id = identifier

      if match(Token::LEFT_BRACE)
        return Ast::VarDefine.new(id, block)
      else
        return Ast::VarFetch.new(id)
      end
    end

    def timer
      times = []

      until check(Token::LEFT_BRACE)
        times.concat(time)
      end

      consume(Token::LEFT_BRACE, "Expect '{' after time clause")

      return Ast::TimeBlock.new(times, block)
    end

    def time
      consume(Token::AT, "Time clause must start with '@'")

      keyword = time_keyword

      return [Ast::Time.new(keyword, 0)] if keyword == Token::SNAP

      consume(Token::NUMBER, "Expected number after time directive")
      value = previous.literal
      consume(Token::SECONDS, "Expected seconds 's' after number")

      times = []

      if keyword == Token::FADE || keyword == Token::DELAY
        down_value = nil
        if match(Token::SLASH)
          consume(Token::NUMBER, "Expected number after time directive")
          down_value = previous.literal
          consume(Token::SECONDS, "Expected seconds 's' after number")
        end

        if !down_value.nil? && keyword == Token::FADE
          times << Ast::Time.new(Token::FADE_UP, value)
          times << Ast::Time.new(Token::FADE_DOWN, down_value)
        elsif !down_value.nil? && keyword == Token::DELAY
          times << Ast::Time.new(Token::DELAY_UP, value)
          times << Ast::Time.new(Token::DELAY_DOWN, down_value)
        else
          times << Ast::Time.new(keyword, value)
        end
      end

      times
    end

    def time_keyword
      if match(*TIME_KEYWORDS)
        return previous.lexeme
      end

      error(peek, "Invalid time keyword")
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

      val, times = arguments

      consume(Token::SEMICOLON, "Expected semicolon after apply")

      if times.empty?
        return Ast::Apply.new(param, val)
      else
        block = Ast::Block.new([Ast::Apply.new(param, val)])
        return Ast::TimeBlock.new(times, block)
      end
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
      times = []

      until check(Token::SEMICOLON)
        if check(Token::AT)
          times.concat(time)
        else
          args << argument
        end
      end

      [args, times]
    end

    def argument
      val = nil

      if match(Token::UNDERSCORE)
        val = Ast::Value.new(nil)
      elsif match(Token::IDENTIFIER)
        val = call
      elsif match(Token::LEFT_PAREN)
        val = tuple
      else
        val = value

        if check(Token::ARROW)
          val = range(val)
        end
      end

      return val
    end

    def call
      id = previous.lexeme
      args = []

      consume(Token::LEFT_PAREN, "Call must start with '('.")

      until check(Token::RIGHT_PAREN)
        args << argument

        # discard any seconds we find in the arguments
        match(Token::SECONDS)
        
        unless check(Token::RIGHT_PAREN)
          consume(Token::COMMA, "Arguments must be seperated by a comma")
        end
      end

      consume(Token::RIGHT_PAREN, "Function call must be closed with ')'")

      return Ast::Call.new(id, args)
    end

    def tuple
      literal = {}
      index = 0

      until check(Token::RIGHT_PAREN)
        if match(Token::IDENTIFIER)
          id = previous.lexeme.to_sym
          consume(Token::COLON, "Expected colon after identifier")
          literal[id] = argument
        elsif check(Token::NUMBER) || check(Token::UNDERSCORE)
          literal[:"_#{index}"] = argument
          index += 1
        end

        # all values end with comma unless this is the last one
        unless check(Token::RIGHT_PAREN)
          consume(Token::COMMA, "Values in tuple must be seperated by commas")
        end
      end

      consume(Token::RIGHT_PAREN, "Tuple must be closed with ')'.")

      return Ast::Tuple.new(literal)
    end

    def value
      if match(Token::NUMBER)
        return Ast::Value.new(previous.literal)
      elsif match(Token::HASH)
        return variable
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
          select =  Ast::Selector.new(range(first))
        end

        consume(Token::RIGHT_BRACKET, "Expect ']' to end selector")
        return select
      end

      error(peek, "Expected valid selector")
    end

    def range(first)
      consume(Token::ARROW, "Expect -> for range.")

      return Ast::Range.new(first, value)

      error(peek, "Expected ending value for a range")
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
end
