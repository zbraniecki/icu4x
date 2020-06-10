use super::ast;

#[derive(Debug)]
pub enum Token {
    Operand(ast::Operand),
    Operator(ast::Operator),
    Number(usize),
    Dot,
    DotDot,
    Comma,
    Or,
    And,
    Modulo,
}

pub struct Lexer<'l> {
    input: &'l [u8],
    ptr: usize,
    error_ptr: Option<usize>,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l [u8]) -> Self {
        Self {
            input,
            ptr: 0,
            error_ptr: None,
        }
    }

    fn expect(&mut self, b: u8) -> bool {
        if Some(&b) == self.input.get(self.ptr) {
            self.ptr += 1;
            true
        } else {
            self.error_ptr = Some(self.ptr - 1);
            false
        }
    }

    fn next(&mut self) -> Option<Token> {
        while let Some(b) = self.input.get(self.ptr) {
            self.ptr += 1;
            match b {
                b' ' => {}
                b'o' => {
                    if self.expect(b'r') {
                        return Some(Token::Or);
                    } else {
                        return None;
                    }
                }
                b'a' => {
                    if self.expect(b'n') && self.expect(b'd') {
                        return Some(Token::And);
                    } else {
                        return None;
                    }
                }
                b'n' => return Some(Token::Operand(ast::Operand::N)),
                b'i' => return Some(Token::Operand(ast::Operand::I)),
                b'f' => return Some(Token::Operand(ast::Operand::F)),
                b't' => return Some(Token::Operand(ast::Operand::T)),
                b'v' => return Some(Token::Operand(ast::Operand::V)),
                b'w' => return Some(Token::Operand(ast::Operand::W)),
                b'=' => return Some(Token::Operator(ast::Operator::Eq)),
                b'!' => {
                    if self.expect(b'=') {
                        return Some(Token::Operator(ast::Operator::NotEq));
                    } else {
                        return None;
                    }
                }
                b'.' => {
                    if let Some(b'.') = self.input.get(self.ptr) {
                        self.ptr += 1;
                        return Some(Token::DotDot);
                    } else {
                        return Some(Token::Dot);
                    }
                }
                b',' => return Some(Token::Comma),
                b'0'..=b'9' => {
                    let start = self.ptr - 1;
                    while let Some(b'0'..=b'9') = self.input.get(self.ptr) {
                        self.ptr += 1;
                    }
                    let end = self.ptr;

                    let mut value = 0;
                    for i in start..end {
                        let digit = (self.input[i] - b'0') as usize;
                        let mult = 10usize.pow((end - i) as u32 - 1);
                        value += digit * mult;
                    }
                    return Some(Token::Number(value as usize));
                }
                b'%' => return Some(Token::Modulo),
                _ => unimplemented!(),
            };
        }
        None
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
