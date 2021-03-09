use crate::error::ParserError;
use icu_string::Slice;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

#[derive(PartialEq, Debug, Clone)]
pub enum PlaceholderElement<P, S> {
    Placeholder(P),
    Literal(S),
}

impl<P, S> From<S> for PlaceholderElement<P, S> {
    fn from(input: S) -> Self {
        Self::Literal(input)
    }
}

#[derive(PartialEq)]
enum ParserState {
    Default,
    Placeholder,
    QuotedLiteral,
}

pub struct Parser<'s, S, P> {
    input: &'s S,
    start_idx: usize,
    idx: usize,
    len: usize,
    state: ParserState,
    marker: PhantomData<&'s P>,
}

impl<'s, S, P> Parser<'s, S, P> {
    pub fn new(input: &'s S) -> Self
    where
        S: Slice,
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
}

macro_rules! handle_literal {
    ($self:expr, $ty:expr) => {{
        $self.state = $ty;
        let range = $self.start_idx..$self.idx;
        $self.start_idx = $self.idx + 1;
        $self.idx += 1;
        if !range.is_empty() {
            return Some(Ok($self.input.get_slice(range).into()));
        } else {
            continue;
        }
    }};
}

impl<'s, S, P> Iterator for Parser<'s, S, P>
where
    S: Slice,
    P: FromStr + Display,
    P::Err: Debug,
{
    type Item = Result<PlaceholderElement<P, S>, ParserError<<P as FromStr>::Err>>;

    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        while let Some(b) = self.input.get_byte(self.idx) {
            match self.state {
                ParserState::Placeholder if b == b'}' => {
                    self.state = ParserState::Default;
                    let range = self.start_idx..self.idx;
                    self.start_idx = self.idx + 1;
                    self.idx += 1;
                    match self.input.get_str_slice(range).parse() {
                        Ok(ret) => {
                            return Some(Ok(PlaceholderElement::Placeholder(ret)));
                        }
                        Err(err) => {
                            return Some(Err(ParserError::InvalidPlaceholder(err)));
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
            ParserState::Placeholder => Some(Err(ParserError::UnclosedPlaceholder)),
            ParserState::QuotedLiteral => Some(Err(ParserError::UnclosedQuotedLiteral)),
            ParserState::Default => {
                let range = self.start_idx..self.len;
                self.start_idx = self.len;
                if !range.is_empty() {
                    Some(Ok(PlaceholderElement::Literal(self.input.get_slice(range))))
                } else {
                    None
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
        let iter = Parser::<_, usize>::new(&"{0} and {1} FOO");
        let v: Result<Vec<_>, _> = iter.collect();
        assert_eq!(
            v.unwrap(),
            vec![
                PlaceholderElement::Placeholder(0),
                PlaceholderElement::Literal(" and ".into()),
                PlaceholderElement::Placeholder(1),
                PlaceholderElement::Literal(" FOO".into()),
            ]
        );
    }
}
