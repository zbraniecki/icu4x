#[derive(Debug, PartialEq)]
pub struct Literal<'s> {
    value: &'s str,
    quotes: Vec<usize>,
}

impl<'s> Literal<'s> {
    pub fn new(value: &'s str) -> Self {
        Self {
            value,
            quotes: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Element<'s> {
    Placeholder(&'s str),
    Literal(Literal<'s>),
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self { }
    }

    pub fn parse<'l>(&self, input: &'l str) -> Vec<Element<'l>> {
        let mut result = vec![];
        let mut in_placeholder = false;
        let mut quotes = vec![];
        let mut start = 0;

        let mut bytes = input.bytes().enumerate();
        while let Some((idx, b)) = bytes.next() {
            if in_placeholder {
                if b == b'}' {
                    assert!(idx != start);
                    result.push(Element::Placeholder(&input[start..idx]));
                    in_placeholder = false;
                    start = idx + 1;
                }
            } else {
                if b == b'{' {
                    if start != idx {
                        result.push(Element::Literal(Literal {
                            value: &input[start..idx],
                            quotes
                        }));
                        quotes = vec![];
                    }
                    in_placeholder = true;
                    start = idx + 1;
                } else if b == b'\'' {
                    quotes.push(idx - start);
                    bytes.next();
                }
            }
        }
        result
    }
}

use std::fmt::{Error, Write};

//XXX: We need a way to produce an interpolated list of elements, rather than a string
pub fn write_format<W: Write>(f: &mut W, pattern: &[Element], replacements: &[&str]) -> Result<(), Error> {
    for element in pattern {
        match element {
            Element::Literal(s) => {
                if s.quotes.is_empty() {
                    f.write_str(s.value)?;
                } else {
                    let mut start = 0;
                    for idx in &s.quotes {
                        if start != *idx {
                            f.write_str(&s.value[start..*idx])?;
                        }
                        start = idx + 1;
                    }
                    if start < s.value.len() {
                        f.write_str(&s.value[start..])?;
                    }
                }
            },
            Element::Placeholder(s) => {
                f.write_str(s)?;
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
        let parser = Parser::new();
        let ast = parser.parse("Foo {0} and {1}");
        assert_eq!(ast, vec![
            Element::Literal(Literal { value: "Foo ", quotes: vec![] }),
            Element::Placeholder("0"),
            Element::Literal(Literal { value: " and ", quotes: vec![] }),
            Element::Placeholder("1"),
        ]);

        let parser = Parser::new();
        let ast = parser.parse("{start}, {middle} and {end}");
        assert_eq!(ast, vec![
            Element::Placeholder("start"),
            Element::Literal(Literal { value: ", ", quotes: vec![] }),
            Element::Placeholder("middle"),
            Element::Literal(Literal { value: " and ", quotes: vec![] }),
            Element::Placeholder("end"),
        ]);

        let parser = Parser::new();
        let ast = parser.parse("{0} 'at' {1}");
        let mut s = String::new();
        write_format(&mut s, &ast, &["Hello", "World"]);
        assert_eq!(s, "0 at 1");

        assert_eq!(ast, vec![
            Element::Placeholder("0"),
            Element::Literal(Literal { value: " 'at' ", quotes: vec![1, 4] }),
            Element::Placeholder("1"),
        ]);
    }
}
