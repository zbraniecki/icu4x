use crate::error::ParserError;
use icu_string::Slice;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

#[derive(PartialEq, Debug, Clone)]
pub enum PlaceholderElement<'s, P> {
    Placeholder(P),
    Literal(Cow<'s, str>),
}

#[derive(PartialEq)]
enum ParserState {
    Default,
    Placeholder,
    QuotedLiteral,
}

pub struct Parser<'p, 's, S, P> {
    input: &'p S,
    start_idx: usize,
    idx: usize,
    len: usize,
    state: ParserState,
    marker: PhantomData<&'s P>,
}

macro_rules! handle_literal {
    ($self:expr, $ty:expr) => {{
        $self.state = $ty;
        let range = $self.start_idx..$self.idx;
        $self.start_idx = $self.idx + 1;
        $self.idx += 1;
        if !range.is_empty() {
            return Ok(Some(PlaceholderElement::Literal(
                $self.input.get_cow_slice(range),
            )));
        } else {
            continue;
        }
    }};
}

impl<'p, 's, S, P> Parser<'p, 's, S, P> {
    pub fn new(input: &'p S) -> Self
    where
        S: Slice<'s, Cow<'s, str>>,
    {
        let len = input.length();
        Self {
            input,
            start_idx: 0,
            idx: 0,
            len,
            state: ParserState::Default,
            marker: std::marker::PhantomData,
        }
    }

    pub fn try_next(
        &mut self,
    ) -> Result<Option<PlaceholderElement<'s, P>>, ParserError<<P as FromStr>::Err>>
    where
        S: Slice<'s, Cow<'s, str>>,
        P: FromStr + Display,
        P::Err: Debug,
    {
        while let Some(b) = self.input.get_byte(self.idx) {
            match self.state {
                ParserState::Placeholder if b == b'}' => {
                    self.state = ParserState::Default;
                    let range = self.start_idx..self.idx;
                    self.start_idx = self.idx + 1;
                    self.idx += 1;
                    match self.input.get_str_slice(range).parse() {
                        Ok(ret) => {
                            return Ok(Some(PlaceholderElement::Placeholder(ret)));
                        }
                        Err(err) => {
                            return Err(ParserError::InvalidPlaceholder(err));
                        }
                    }
                }
                ParserState::QuotedLiteral if b == b'\'' => {
                    handle_literal!(self, ParserState::Default)
                }
                ParserState::Default if b == b'{' => {
                    handle_literal!(self, ParserState::Placeholder)
                }
                ParserState::Default if b == b'\'' => {
                    handle_literal!(self, ParserState::QuotedLiteral)
                }
                _ => {
                    self.idx += 1;
                }
            }
        }
        match self.state {
            ParserState::Placeholder => Err(ParserError::UnclosedPlaceholder),
            ParserState::QuotedLiteral => Err(ParserError::UnclosedQuotedLiteral),
            ParserState::Default => {
                let range = self.start_idx..self.len;
                self.start_idx = self.len;
                if !range.is_empty() {
                    Ok(Some(PlaceholderElement::Literal(
                        self.input.get_cow_slice(range),
                    )))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse() {
        let input: Cow<str> = "{0} and {1} FOO".into();
        let mut iter = Parser::new(&input);
        let mut result = vec![];
        while let Some(elem) = iter.try_next().unwrap() {
            result.push(elem);
        }
        assert_eq!(
            result,
            vec![
                PlaceholderElement::Placeholder(0),
                PlaceholderElement::Literal(" and ".into()),
                PlaceholderElement::Placeholder(1),
                PlaceholderElement::Literal(" FOO".into()),
            ]
        );
    }
}
