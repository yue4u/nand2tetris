use std::str::Chars;
#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    literal: Literal<'a>,
}

impl<'a> Token<'a> {
    pub fn xml(&self) -> String {
        let escaped_content = self
            .literal
            .to_string()
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace(r#"""#, "&quot;")
            .replace("'", "&apos;");
        match self.kind.as_str() {
            Some(kind_str) => format!("<{0}> {1} </{0}>", kind_str, escaped_content),
            None => "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Literal<'a> {
    Integer(i16),
    String(&'a str),
}

impl<'a> ToString for Literal<'a> {
    fn to_string(&self) -> String {
        match self {
            Literal::String(val) => val.to_string(),
            Literal::Integer(val) => val.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Comment,
    IntegerConstant,
    StringConstant,
    Identifier,
    Whitespace,
    // Keyword
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    // Symbol
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    Or,
    LT,
    GT,
    EQ,
    Not,
}

impl std::str::FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "class" => Ok(TokenKind::Class),
            "constructor" => Ok(TokenKind::Constructor),
            "function" => Ok(TokenKind::Function),
            "method" => Ok(TokenKind::Method),
            "field" => Ok(TokenKind::Field),
            "static" => Ok(TokenKind::Static),
            "var" => Ok(TokenKind::Var),
            "int" => Ok(TokenKind::Int),
            "char" => Ok(TokenKind::Char),
            "boolean" => Ok(TokenKind::Boolean),
            "void" => Ok(TokenKind::Void),
            "true" => Ok(TokenKind::True),
            "false" => Ok(TokenKind::False),
            "null" => Ok(TokenKind::Null),
            "this" => Ok(TokenKind::This),
            "let" => Ok(TokenKind::Let),
            "do" => Ok(TokenKind::Do),
            "if" => Ok(TokenKind::If),
            "else" => Ok(TokenKind::Else),
            "while" => Ok(TokenKind::While),
            "return" => Ok(TokenKind::Return),
            _ => Err(()),
        }
    }
}

use TokenKind::*;

impl TokenKind {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Class | Constructor | Function | Method | Field | Static | Var | Int | Char
            | Boolean | Void | True | False | Null | This | Let | Do | If | Else | While
            | Return => Some("keyword"),
            LBrace | RBrace | LParen | RParen | LBracket | RBracket | Dot | Comma | Semicolon
            | Plus | Minus | Asterisk | Slash | And | Or | LT | GT | EQ | Not => Some("symbol"),
            IntegerConstant => Some("integerConstant"),
            StringConstant => Some("stringConstant"),
            Identifier => Some("identifier"),
            _ => None,
        }
    }
}

pub struct Tokenizer<'a> {
    source_len: usize,
    chars: Chars<'a>,
    read_pos: usize,
    source: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn xml(&'a mut self) -> String {
        let mut out = vec!["<tokens>".to_string()];
        for token in self.tokenize() {
            let line = token.xml();
            if !line.is_empty() {
                out.push(line);
            }
        }
        out.push("</tokens>".to_string());
        out.join("\n")
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            source_len: source.len(),
            chars: source.chars(),
            read_pos: 0,
            source,
        }
    }

    pub fn tokenize(&'a mut self) -> impl Iterator<Item = Token> {
        std::iter::from_fn(move || self.first_token())
    }

    fn peek_char(&self) -> char {
        self.chars.clone().nth(0).unwrap_or('\0')
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
    fn eat(&mut self, predicate: char) -> bool {
        if self.peek_char() == predicate {
            self.chars.next();
            return true;
        }
        false
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_char()) && !self.is_eof() {
            self.chars.next();
        }
    }

    fn eat_until(&mut self, predicate: char) {
        while self.peek_char() != predicate && !self.is_eof() {
            self.chars.next();
        }
        self.chars.next();
    }
    fn current_span(&mut self) -> (&'a str, usize) {
        let len = self.source_len - self.read_pos - self.chars.as_str().len();
        (&self.source[self.read_pos..self.read_pos + len], len)
    }

    fn take_span(&mut self) -> &'a str {
        let (span, len) = self.current_span();
        self.read_pos += len;
        span
    }

    fn first_token(&mut self) -> Option<Token<'a>> {
        if self.source.is_empty() {
            return None;
        }

        let kind = match self.chars.next()? {
            '/' => match self.peek_char() {
                '/' => {
                    self.eat_until('\n');
                    Comment
                }
                _ if self.eat('*') => {
                    while !(self.is_eof() || (self.eat('*') && self.eat('/'))) {
                        self.chars.next();
                    }
                    // println!("current_span: {}", self.current_span().0.to_string());
                    Comment
                }
                _ => Slash,
            },
            '{' => LBrace,
            '}' => RBrace,
            '(' => LParen,
            ')' => RParen,
            '[' => LBracket,
            ']' => RBracket,
            '.' => Dot,
            ',' => Comma,
            ';' => Semicolon,
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '&' => And,
            '|' => Or,
            '<' => LT,
            '>' => GT,
            '=' => EQ,
            '~' => Not,
            '0'..='9' => {
                self.eat_while(|c| c.is_ascii_digit());
                IntegerConstant
            }
            '"' => {
                self.eat_while(|c| match c {
                    '"' => false,
                    '\n' => panic!("new line is not allowed in jack string"),
                    _ => true,
                });
                self.chars.next();
                StringConstant
            }
            c if is_identifier_start(c) => {
                self.eat_while(is_identifier_tail);
                match self.current_span().0.parse::<TokenKind>() {
                    Ok(kind) => kind,
                    _ => TokenKind::Identifier,
                }
            }
            c if is_whitespace(c) => {
                self.eat_while(is_whitespace);
                TokenKind::Whitespace
            }
            unknown => panic!("unknown token type of {}", unknown),
        };
        let span = self.take_span();
        let literal = match kind {
            IntegerConstant => Literal::Integer(
                span.parse::<i16>()
                    .expect(&format!("jack only support i16 int but found {}", span)),
            ),
            StringConstant => Literal::String(&span[1..span.len() - 1]),
            _ => Literal::String(span),
        };
        let token = Token { kind, literal };
        // println!("{:?}", token);
        Some(token)
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_tail(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// from https://github.com/rust-lang/rust/blob/fc93e4719c2ced744d75f0c281bb7ba29844bedd/compiler/rustc_lexer/src/lib.rs#L235
fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_whitespace() {
        let mut tokenizer = Tokenizer::new(" ");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::Whitespace);
        assert_eq!(token.literal.to_string(), " ");
    }

    #[test]
    fn test_comment() {
        let mut tokenizer = Tokenizer::new("// class");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::Comment);
        let mut tokenizer = Tokenizer::new("/* class */");
        let mut stream = tokenizer.tokenize();
        let token = stream.next().unwrap();
        assert_eq!(token.kind, TokenKind::Comment);
        assert_eq!(stream.next().is_none(), true);
    }

    #[test]
    fn test_keyword() {
        let mut tokenizer = Tokenizer::new("class");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::Class);
        assert_eq!(token.xml(), "<keyword> class </keyword>");
    }

    #[test]
    fn test_symbol() {
        let mut tokenizer = Tokenizer::new("*");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::Asterisk);
        assert_eq!(token.xml(), "<symbol> * </symbol>".to_string());
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        Tokenizer::new("!").tokenize().next();
    }

    #[test]
    fn test_integer_constant() {
        let mut tokenizer = Tokenizer::new("1");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::IntegerConstant);
        assert_eq!(token.literal, Literal::Integer(1));
        assert_eq!(token.xml(), "<integerConstant> 1 </integerConstant>");

        let mut tokenizer = Tokenizer::new("1");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::IntegerConstant);
        assert_eq!(token.literal, Literal::Integer(1));
        assert_eq!(token.xml(), "<integerConstant> 1 </integerConstant>");

        let mut tokenizer = Tokenizer::new("32767");
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::IntegerConstant);
        assert_eq!(token.literal, Literal::Integer(32767));
        assert_eq!(token.xml(), "<integerConstant> 32767 </integerConstant>");
    }

    #[test]
    #[should_panic]
    fn test_integer_constant_panic_on_out_of_range() {
        let mut tokenizer = Tokenizer::new("32768");
        tokenizer.tokenize().next();
    }

    #[test]
    fn test_string_constant() {
        let mut tokenizer = Tokenizer::new(r#""""#);
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::StringConstant);
        assert_eq!(token.literal, Literal::String(""));
        assert_eq!(
            token.xml(),
            "<stringConstant>  </stringConstant>".to_string()
        );

        let mut tokenizer = Tokenizer::new(r#""TEST STRING""#);
        let token = tokenizer.tokenize().next().unwrap();
        assert_eq!(token.kind, TokenKind::StringConstant);
        assert_eq!(token.literal, Literal::String("TEST STRING"));
        assert_eq!(
            token.xml(),
            "<stringConstant> TEST STRING </stringConstant>".to_string()
        );
    }

    use std::ffi::OsStr;
    use std::path::PathBuf;

    #[cfg(test)]
    fn compare(name: &str) {
        let mut jack_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(OsStr::new("../../projects/10/"));
        let mut xml_path = jack_path.clone();
        jack_path.push(name);
        jack_path.set_extension("jack");
        xml_path.push(format!("{}T", name));
        xml_path.set_extension("xml");
        let jack = std::fs::read_to_string(jack_path)
            .expect("failed to read test file")
            .to_string();
        let xml_content = std::fs::read_to_string(xml_path)
            .expect("failed to read test file")
            .to_string();
        let output_content = Tokenizer::new(&jack).xml();
        // compare line by line to ignore the different between newline character
        let mut tokens = output_content.lines();
        for xml_line in xml_content.lines() {
            if xml_line.is_empty() {
                continue;
            }
            let token = tokens.next().unwrap();
            assert_eq!(xml_line, token);
        }
    }

    #[test]
    fn test_array_test() {
        compare("ArrayTest/Main");
    }
    #[test]
    fn test_expression_less_square_main() {
        compare("ExpressionLessSquare/Main");
        compare("ExpressionLessSquare/SquareGame");
        compare("ExpressionLessSquare/Square");
    }

    #[test]
    fn test_square_main() {
        compare("Square/Main");
        compare("Square/SquareGame");
        compare("Square/Square");
    }
}
