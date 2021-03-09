use crate::{error::ParserError, parser::PlaceholderElement};
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

pub fn interpolate<'s, I, R, K, E>(
    pattern: I,
    mut replacements: R,
) -> impl Iterator<Item = ParserResult<E, <K as FromStr>::Err>>
where
    I: IntoIterator<Item = ParserResult<PlaceholderElement<'s, K>, <K as FromStr>::Err>>,
    K: FromStr + std::fmt::Display,
    K::Err: std::fmt::Debug,
    R: ReplacementProvider<K, E>,
    E: From<Cow<'s, str>>,
{
    let mut current_placeholder: Option<R::Iter> = None;
    let mut pattern = pattern.into_iter();

    std::iter::from_fn(move || loop {
        if let Some(ref mut placeholder) = &mut current_placeholder {
            match placeholder.next() {
                Some(v) => {
                    return Some(Ok(v));
                }
                None => {
                    current_placeholder = None;
                }
            }
        }
        if let Some(element) = pattern.next() {
            match element {
                Ok(PlaceholderElement::Literal(l)) => {
                    return Some(Ok(l.into()));
                }
                Ok(PlaceholderElement::Placeholder(p)) => match replacements.take_replacement(&p) {
                    Some(p) => {
                        current_placeholder = Some(p);
                        continue;
                    }
                    None => return Some(Err(ParserError::MissingPlaceholder(p.to_string()))),
                },
                Err(err) => {
                    return Some(Err(err));
                }
            }
        } else {
            return None;
        }
    })
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
            let mut i = interpolate(iter, replacements);
            let result = i
                .try_fold(String::new(), |mut acc, t| {
                    if t.map(|t| write!(acc, "{}", t)).is_err() {
                        Err(())
                    } else {
                        Ok(acc)
                    }
                })
                .unwrap();
            assert_eq!(result, sample.2);
        }
    }
}
