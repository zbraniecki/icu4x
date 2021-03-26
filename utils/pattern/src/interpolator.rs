use crate::{
    error::ParserError,
    parser::{Parser, PlaceholderElement},
};
use std::{borrow::Cow, collections::HashMap, str::FromStr};

type ParserResult<E, R> = std::result::Result<E, ParserError<R>>;

pub trait ReplacementProvider<K, E> {
    type Iter: Iterator<Item = E>;

    fn take_replacement(&mut self, key: &K) -> Option<Self::Iter>;
}

impl<E> ReplacementProvider<usize, E> for Vec<Vec<E>> {
    type Iter = <Vec<E> as IntoIterator>::IntoIter;

    fn take_replacement(&mut self, input: &usize) -> Option<Self::Iter> {
        let r = self.get_mut(*input)?;
        Some(std::mem::take(r).into_iter())
    }
}

impl<E> ReplacementProvider<String, E> for HashMap<String, Vec<E>> {
    type Iter = <Vec<E> as IntoIterator>::IntoIter;

    fn take_replacement(&mut self, input: &String) -> Option<Self::Iter> {
        let r = self.remove(input)?;
        Some(r.into_iter())
    }
}

pub struct Interpolator<'i, 's, R, K, E>
where
    R: ReplacementProvider<K, E>,
{
    pattern: Parser<'i, 's, Cow<'s, str>, K>,
    replacements: R,
    current_placeholder: Option<R::Iter>,
}

impl<'i, 's, R, K, E> Interpolator<'i, 's, R, K, E>
where
    R: ReplacementProvider<K, E>,
{
    pub fn new(pattern: Parser<'i, 's, Cow<'s, str>, K>, replacements: R) -> Self {
        Self {
            pattern,
            replacements,
            current_placeholder: None,
        }
    }

    pub fn try_next(&mut self) -> ParserResult<Option<E>, <K as FromStr>::Err>
    where
        E: From<Cow<'s, str>>,
        K: FromStr + std::fmt::Display,
        K::Err: std::fmt::Debug,
    {
        loop {
            if let Some(ref mut placeholder) = &mut self.current_placeholder {
                match placeholder.next() {
                    Some(v) => {
                        return Ok(Some(v));
                    }
                    None => {
                        self.current_placeholder = None;
                    }
                }
            }
            match self.pattern.try_next() {
                Ok(element) => match element {
                    Some(PlaceholderElement::Literal(l)) => {
                        return Ok(Some(l.into()));
                    }
                    Some(PlaceholderElement::Placeholder(p)) => match self.replacements.take_replacement(&p)
                    {
                        Some(p) => {
                            self.current_placeholder = Some(p);
                            continue;
                        }
                        None => return Err(ParserError::MissingPlaceholder(p.to_string())),
                    },
                    None => {
                        return Ok(None);
                    }
                },
                Err(err) => return Err(err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;
    use std::fmt::{Display, Write};

    const SAMPLES: &[(&str, &[&[&str]], &str)] = &[
        (
            "Foo {0} and {1}",
            &[&["Hello"], &["World"]],
            "Foo Hello and World",
        ),
        (
            "{0}, {1} and {2}",
            &[&["Start"], &["Middle"], &["End"]],
            "Start, Middle and End",
        ),
        ("{0} 'at' {1}", &[&["Hello"], &["World"]], "Hello at World"),
    ];

    pub enum Element<'s> {
        Literal(Cow<'s, str>),
    }

    impl Display for Element<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Literal(s) => f.write_str(s),
            }
        }
    }

    impl<'s> From<Cow<'s, str>> for Element<'s> {
        fn from(input: Cow<'s, str>) -> Self {
            Self::Literal(input)
        }
    }

    impl<'s> From<&'s str> for Element<'s> {
        fn from(input: &'s str) -> Self {
            Self::Literal(input.into())
        }
    }

    #[test]
    fn simple_interpolate() {
        for sample in SAMPLES.iter() {
            let input = sample.0.into();
            let iter = Parser::new(&input);

            let replacements: Vec<Vec<Element>> = sample
                .1
                .iter()
                .map(|r| r.iter().map(|&t| t.into()).collect())
                .collect();
            let mut i = Interpolator::new(iter, replacements);
            let mut result = String::new();
            while let Some(elem) = i.try_next().unwrap() {
                write!(result, "{}", elem).unwrap();
            }
            assert_eq!(result, sample.2);
        }
    }
}
