require "ruby-enum"
class Token
  include Ruby::Enum

  # Single character tokens
  define :LEFT_PAREN, 'LEFT_PAREN'
  define :RIGHT_PAREN, 'RIGHT_PAREN'
  define :LEFT_BRACE, 'LEFT_BRACE'
  define :RIGHT_BRACE, 'RIGHT_BRACE'
  define :LEFT_BRACKET, 'LEFT_BRACKET'
  define :RIGHT_BRACKET, 'RIGHT_BRACKET'
  define :COMMA, 'COMMA'
  # define :DOT, 'DOT'
  define :MINUS, 'MINUS'
  # define :PLUS, 'PLUS'
  define :SEMICOLON, 'SEMICOLON'
  define :SLASH, 'SLASH'
  # define :STAR, 'STAR'
  define :COLON, 'COLON'
  define :AT, 'AT'
  define :SECONDS, 'SECONDS'
  define :UNDERSCORE, 'UNDERSCORE'

  # One or two character tokens
  # define :BANG, 'BANG'
  # define :BANG_EQUAL, 'BANG_EQUAL'
  # define :EQUAL, 'EQUAL'
  # define :EQUAL_EQUAL, 'EQUAL_EQUAL'
  # define :GREATER, 'GREATER'
  # define :GREATER_EQUAL, 'GREATER_EQUAL'
  # define :LESS, 'LESS'
  # define :LESS_EQUAL, 'LESS_EQUAL'
  define :ARROW, 'ARROW'

  # Literals
  define :IDENTIFIER, 'IDENTIFIER'
  define :STRING, 'STRING'
  define :NUMBER, 'NUMBER'

  # Keywords
  define :FADE, 'fade'
  define :FADE_UP, 'up'
  define :FADE_DOWN, 'down'
  define :DELAY, 'delay'
  define :DELAY_UP, 'dup'
  define :DELAY_DOWN, 'ddown'
  define :SNAP, 'snap'

  # define :AND, 'and'
  # define :CLASS, 'class'
  # define :ELSE, 'else'
  # define :FALSE, 'false'
  # define :FUN, 'fun'
  # define :FOR, 'for'
  # define :IF, 'if'
  # define :NIL, 'nil'
  # define :OR, 'or'
  # define :PRINT, 'print'
  # define :RETURN, 'return'
  # define :SUPER, 'super'
  # define :THIS, 'this'
  # define :TRUE, 'true'
  # define :VAR, 'var'
  # define :WHILE, 'while'

  define :EOF, 'EOF'
end

TIME_KEYWORDS = [
  Token::FADE,
  Token::FADE_UP,
  Token::FADE_DOWN,
  Token::DELAY,
  Token::DELAY_UP,
  Token::DELAY_DOWN,
  Token::SNAP
]
