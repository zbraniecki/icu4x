use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Element {
    Placeholder(String),
    Literal(String),
}

#[derive(Debug, PartialEq)]
pub enum Element2 {
    Placeholder(Range<usize>),
    Literal(Range<usize>),
}

pub struct Parser {
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn parse(self) -> Vec<Element> {
        let mut result = vec![];
        let mut current = Element::Literal(String::new());
        for ch in self.input.chars() {
            match &mut current {
                Element::Literal(s) => {
                    if ch == '{' {
                        if !s.is_empty() {
                            result.push(current);
                        }
                        current = Element::Placeholder(String::new());
                    } else {
                        s.push(ch);
                    }
                },
                Element::Placeholder(s) => {
                    if ch == '}' {
                        assert!(!s.is_empty());
                        result.push(current);
                        current = Element::Literal(String::new());
                    } else {
                        s.push(ch);
                    }
                }
            }
        }
        result
    }
}

pub struct Parser2 {}

impl Parser2 {
    pub fn new() -> Self {
        Self { }
    }

    pub fn parse(&self, input: &str) -> Vec<Element2> {
        let mut result = vec![];
        let mut current = Element2::Literal(0..0);
        for ch in input.chars() {
            match &mut current {
                Element2::Literal(s) => {
                    if ch == '{' {
                        if s.start != s.end {
                            result.push(current);
                        }
                        current = Element2::Placeholder(0..0);
                    } else {
                        s.end += 1;
                    }
                },
                Element2::Placeholder(s) => {
                    if ch == '}' {
                        assert!(s.start != s.end);
                        result.push(current);
                        current = Element2::Literal(0..0);
                    } else {
                        s.end += 1;
                    }
                }
            }
        }
        result
    }
}

use std::fmt::{Error, Write};

pub fn write_format<W: Write>(f: &mut W, pattern: Vec<Element>, replacements: &[&str]) -> Result<(), Error> {
    for element in pattern {
        match element {
            Element::Literal(s) => f.write_str(&s),
            Element::Placeholder(s) => f.write_str(&s),
        };
    }
    Ok(())
}

pub fn write_format2<W: Write>(f: &mut W, input: &str, pattern: Vec<Element2>, replacements: &[&str]) -> Result<(), Error> {
    for element in pattern {
        match element {
            Element2::Literal(r) => {
                f.write_str(&input[r])
            },
            Element2::Placeholder(r) => {
                f.write_str(&input[r])
            },
        };
    }
    Ok(())
}

struct Serializer {}

impl Serializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, input: Vec<Element>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Foo {0} and {1}
    // Foo {1} and {0}
    // DD/MM/yy 'at' HH:mm
    // yyyy.MM.dd G 'at' HH:mm:ss zzz
    // EEE, MMM d, ''yy
    #[test]
    fn it_works() {
        let parser = Parser::new("Foo {0} and {1}".to_string());
        let ast = parser.parse();
        assert_eq!(ast, vec![
            Element::Literal("Foo ".to_string()),
            Element::Placeholder("0".to_string()),
            Element::Literal(" and ".to_string()),
            Element::Placeholder("1".to_string()),
        ]);

        let parser = Parser::new("{start}, {middle} and {end}".to_string());
        let ast = parser.parse();
        assert_eq!(ast, vec![
            Element::Placeholder("start".to_string()),
            Element::Literal(", ".to_string()),
            Element::Placeholder("middle".to_string()),
            Element::Literal(" and ".to_string()),
            Element::Placeholder("end".to_string()),
        ]);
    }
}
