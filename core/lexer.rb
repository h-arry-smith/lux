require_relative "token"

class Lexer
  attr_reader :tokens

  def initialize()
    @source = nil
    @tokens = []
    @start = 0
    @current = 0
    @line = 1
  end

  def source=(source)
    @source = source
    @tokens = []
    @start = 0
    @current = 0
    @line = 1
  end

  def scan_tokens
    raise RuntimeError, "No source to scan!" if @source.nil?

    while !at_end?
      @start = @current
      scan_token
    end

    @tokens << LexicalToken.new(Token::EOF, "", nil, @line)
  end

  private

  def scan_token
    c = advance
    case c
    when '('
      add_token(Token::LEFT_PAREN)
    when ')'
      add_token(Token::RIGHT_PAREN)
    when '['
      add_token(Token::LEFT_BRACKET)
    when ']'
      add_token(Token::RIGHT_BRACKET)
    when '{'
      add_token(Token::LEFT_BRACE)
    when '}'
      add_token(Token::RIGHT_BRACE)
    when ':'
      add_token(Token::COLON)
    when ';'
      add_token(Token::SEMICOLON)
    when '@'
      add_token(Token::AT)
    when ','
      add_token(Token::COMMA)
    when '#'
      add_token(Token::HASH)
    when '_'
      if !alphanumeric?(peek) 
        add_token(Token::UNDERSCORE)
      else
        identifier
      end
    when '-'
      if peek == '>'
        advance
        add_token(Token::ARROW)
      elsif digit?(peek)
        number
      end
    when '/'
      if peek == '/'
        advance while peek != "\n" && !at_end?
      else
        add_token(Token::SLASH)
      end
    when 's'
      if digit?(peek_previous)
        add_token(Token::SECONDS)
      else
        identifier
      end
    when " " then return
    when "\r" then return
    when "\t" then return
    when "\n"
      @line += 1
    else
      if digit?(c)
        number
      elsif alpha?(c)
        identifier
      else
        raise LexerError.new(c, @line)
      end
    end
  end

  def number()
    advance while digit?(peek)

    if peek == "." && digit?(peek_next)
      advance
      advance while digit?(peek)
    end

    value = @source[@start...@current].to_f
    add_token(Token::NUMBER, value)
  end

  def identifier
    advance while alphanumeric?(peek)

    text = @source[@start...@current]
    type = Token.key(text)
    type = :IDENTIFIER if type.nil?

    add_token(Token.value(type), text)
  end

  def peek
    "\0" if at_end?
    @source[@current]
  end

  def peek_next
    "\0" if @current + 1 >= @source.length 
    @source[@current + 1]
  end

  def peek_previous
    "" if @current == 0
    @source[@current - 2]
  end

  def advance
    c = @source[@current]
    @current += 1
    c
  end

  def digit?(c)
    c in '0'..'9'
  end

  def alpha?(c)
    ('a'..'z').include?(c) || ('A'..'Z').include?(c) || c == '_'
  end

  def alphanumeric?(c)
    alpha?(c) || digit?(c)
  end

  def add_token(type, literal = nil)
    text = @source[@start...@current]
    token = LexicalToken.new(type, text, literal, @line)

    @tokens << token
  end

  def at_end?
    @current >= @source.length
  end
end

class LexicalToken
  attr_reader :type, :lexeme, :literal, :line

  def initialize(type, lexeme, literal, line)
    @type = type
    @lexeme = lexeme
    @literal = literal
    @line = line
  end

  def to_s
    "#{@type} #{@lexeme} #{@literal}"
  end
end

class LexerError < StandardError
  def initialize(character, line)
    super("[line #{line}] Unexpected character: #{character}")
  end
end
  
