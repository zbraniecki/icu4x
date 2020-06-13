use super::ast;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operand(ast::Operand),
    Operator(ast::Operator),
    Number(u32),
    Dot,
    DotDot,
    DotDotDot,
    Comma,
    Or,
    And,
    Modulo,
    Percent,
    Junk,
    Is,
    Not,
    In,
    Within,
    At,
    Decimal,
    Integer,
    Ellipsis,
    Tilde,
}

pub struct Lexer<'l> {
    chars: &'l [u8],
    ptr: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l [u8]) -> Self {
        Self {
            chars: input,
            ptr: 0,
        }
    }

    fn bump(&mut self) -> Option<&u8> {
        let ret = self.chars.get(self.ptr);
        self.ptr += 1;
        ret
    }

    fn take_if(&mut self, c: u8) -> bool {
        if self.chars.get(self.ptr) == Some(&&c) {
            self.ptr += 1;
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, expected: u8) -> Result<(), ()> {
        if self.bump() == Some(&expected) {
            Ok(())
        } else {
            Err(())
        }
    }

    fn advance_token(&mut self) -> Result<Option<Token>, ()> {
        loop {
            if let Some(c) = self.bump() {
                let token = match c {
                    b' ' => continue,
                    b'n' => {
                        if self.take_if(b'o') {
                            self.expect(b't')?;
                            Token::Not
                        } else {
                            Token::Operand(ast::Operand::N)
                        }
                    }
                    b'i' => match self.chars.get(self.ptr) {
                        Some(b's') => {
                            self.ptr += 1;
                            Token::Is
                        }
                        Some(b'n') => {
                            self.ptr += 1;
                            self.expect(b't')?;
                            self.expect(b'e')?;
                            self.expect(b'g')?;
                            self.expect(b'e')?;
                            self.expect(b'r')?;
                            Token::Integer
                        }
                        _ => Token::Operand(ast::Operand::I),
                    },
                    b'f' => Token::Operand(ast::Operand::F),
                    b't' => Token::Operand(ast::Operand::T),
                    b'v' => Token::Operand(ast::Operand::V),
                    b'w' => {
                        if self.take_if(b'i') {
                            self.expect(b't')?;
                            self.expect(b'h')?;
                            self.expect(b'i')?;
                            self.expect(b'n')?;
                            Token::Within
                        } else {
                            Token::Operand(ast::Operand::W)
                        }
                    }
                    b'm' => {
                        self.expect(b'o')?;
                        self.expect(b'd')?;
                        Token::Modulo
                    }
                    b'=' => Token::Operator(ast::Operator::Eq),
                    b'0'..=b'9' => {
                        let start = self.ptr - 1;

                        while let Some(b'0'..=b'9') = self.chars.get(self.ptr) {
                            self.ptr += 1;
                        }
                        let end = self.ptr;

                        let mut value = 0;
                        for ptr in start..end {
                            let mul = 10u32.pow((end - ptr - 1) as u32);
                            value += ((self.chars[ptr] - b'0') as u32) * mul;
                        }
                        Token::Number(value)
                    }
                    b'a' => {
                        self.expect(b'n')?;
                        self.expect(b'd')?;
                        Token::And
                    }
                    b'o' => {
                        self.expect(b'r')?;
                        Token::Or
                    }
                    b'!' => {
                        self.expect(b'=')?;
                        Token::Operator(ast::Operator::NotEq)
                    }
                    b'.' => {
                        if self.take_if(b'.') {
                            Token::DotDot
                        } else {
                            Token::Dot
                        }
                    }
                    b'd' => {
                        self.expect(b'e')?;
                        self.expect(b'c')?;
                        self.expect(b'i')?;
                        self.expect(b'm')?;
                        self.expect(b'a')?;
                        self.expect(b'l')?;
                        Token::Decimal
                    }
                    b',' => Token::Comma,
                    b'%' => Token::Percent,
                    b'@' => Token::At,
                    226 => {
                        // Ellipsis
                        self.expect(128)?;
                        self.expect(166)?;
                        Token::Ellipsis
                    }
                    b'~' => Token::Tilde,
                    _ => unimplemented!(),
                };
                return Ok(Some(token));
            } else {
                return Ok(None);
            }
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.advance_token().unwrap_or(Some(Token::Junk))
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
