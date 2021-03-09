use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

pub trait Slice<'p> {
    fn get(&self, idx: usize) -> Option<u8>;
    fn get_slice(&self, range: Range<usize>) -> Cow<'p, str>;
    fn length(&self) -> usize;
}

impl<'p> Slice<'p> for Cow<'p, str> {
    fn get(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice(&self, range: Range<usize>) -> Cow<'p, str> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_string()),
        }
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'p> Slice<'p> for &'p str {
    fn get(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice(&self, range: Range<usize>) -> Cow<'p, str> {
        Cow::Borrowed(&self[range])
    }

    fn length(&self) -> usize {
        self.len()
    }
}

#[derive(Debug)]
pub enum ParserError<R> {
    InvalidPlaceholder(R),
    UnknownPlaceholder(String),
    UnclosedPlaceholder,
    UnclosedLiteral,
}

type ParserResult<E, R> = std::result::Result<E, ParserError<R>>;

#[derive(PartialEq, Debug, Clone)]
pub enum PlaceholderElement<'s, P> {
    Placeholder(P),
    Literal(Cow<'s, str>),
}

impl<'s, T> From<&'s str> for PlaceholderElement<'s, T> {
    fn from(literal: &'s str) -> Self {
        Self::Literal(Cow::Borrowed(literal))
    }
}

impl<'s, T> From<String> for PlaceholderElement<'s, T> {
    fn from(literal: String) -> Self {
        Self::Literal(Cow::Owned(literal))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Element<'s, T> {
    Token(T),
    Literal(Cow<'s, str>),
}

impl<'s, T> From<&'s str> for Element<'s, T> {
    fn from(literal: &'s str) -> Self {
        Self::Literal(Cow::Borrowed(literal))
    }
}

impl<'s, T> From<String> for Element<'s, T> {
    fn from(literal: String) -> Self {
        Self::Literal(Cow::Owned(literal))
    }
}

impl<'s, T> std::fmt::Display for Element<'s, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(t) => write!(f, "{}", t),
            Self::Literal(l) => f.write_str(l),
        }
    }
}

#[derive(PartialEq)]
enum ParserState {
    Default,
    Placeholder,
    QuotedLiteral,
}

pub fn parse<'p, S, P>(
    input: S,
) -> impl Iterator<Item = ParserResult<PlaceholderElement<'p, P>, <P as FromStr>::Err>> + 'p
where
    S: Slice<'p> + 'p,
    P: FromStr + std::fmt::Display,
{
    let mut idx = 0;
    let mut start_idx = 0;
    let mut state = ParserState::Default;

    std::iter::from_fn(move || {
        while let Some(ch) = input.get(idx) {
            match state {
                ParserState::Placeholder if ch == b'}' => {
                    state = ParserState::Default;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    idx += 1;
                    match input.get_slice(range).parse() {
                        Ok(ret) => {
                            return Some(Ok(PlaceholderElement::Placeholder(ret)));
                        }
                        Err(err) => {
                            return Some(Err(ParserError::InvalidPlaceholder(err)));
                        }
                    }
                }
                ParserState::QuotedLiteral if ch == b'\'' => {
                    state = ParserState::Default;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    idx += 1;
                    if !range.is_empty() {
                        return Some(Ok(PlaceholderElement::Literal(input.get_slice(range))));
                    } else {
                        continue;
                    }
                }
                ParserState::Default if ch == b'{' => {
                    state = ParserState::Placeholder;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    idx += 1;
                    if !range.is_empty() {
                        return Some(Ok(PlaceholderElement::Literal(input.get_slice(range))));
                    } else {
                        continue;
                    }
                }
                ParserState::Default if ch == b'\'' => {
                    state = ParserState::QuotedLiteral;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    idx += 1;
                    if !range.is_empty() {
                        return Some(Ok(PlaceholderElement::Literal(input.get_slice(range))));
                    } else {
                        continue;
                    }
                }
                _ => {
                    idx += 1;
                }
            }
        }
        match state {
            ParserState::Placeholder => Some(Err(ParserError::UnclosedPlaceholder)),
            ParserState::QuotedLiteral => Some(Err(ParserError::UnclosedLiteral)),
            ParserState::Default => None,
        }
    })
}

pub trait ReplacementProvider<'s, P, T> {
    type Iter: Iterator<Item = Element<'s, T>>;

    fn take_replacement(&mut self, key: &P) -> Option<Self::Iter>;
}

impl<'s, T> ReplacementProvider<'s, usize, T> for Vec<Vec<Element<'s, T>>> {
    type Iter = <Vec<Element<'s, T>> as IntoIterator>::IntoIter;

    fn take_replacement(&mut self, input: &usize) -> Option<Self::Iter> {
        let r = self.get_mut(*input)?;
        Some(std::mem::take(r).into_iter())
    }
}

impl<'s, T> ReplacementProvider<'s, String, T> for HashMap<String, Vec<Element<'s, T>>> {
    type Iter = <Vec<Element<'s, T>> as IntoIterator>::IntoIter;

    fn take_replacement(&mut self, input: &String) -> Option<Self::Iter> {
        let r = self.remove(input)?;
        Some(r.into_iter())
    }
}

pub fn interpolate<'s, I, R, P, T>(
    pattern: I,
    mut replacements: R,
) -> impl Iterator<Item = ParserResult<Element<'s, T>, <P as FromStr>::Err>>
where
    I: IntoIterator<Item = ParserResult<PlaceholderElement<'s, P>, <P as FromStr>::Err>>,
    P: FromStr + std::fmt::Display,
    R: ReplacementProvider<'s, P, T>,
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
                    return Some(Ok(Element::Literal(l)));
                }
                Ok(PlaceholderElement::Placeholder(p)) => match replacements.take_replacement(&p) {
                    Some(mut p) => {
                        if let Some(v) = p.next() {
                            current_placeholder = Some(p);
                            return Some(Ok(v));
                        } else {
                            continue;
                        }
                    }
                    None => return Some(Err(ParserError::UnknownPlaceholder(p.to_string()))),
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
    use std::fmt::Write;

    const samples: &[(&str, &[&[&str]], &str)] = &[
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

    #[derive(Debug)]
    struct Token;

    impl std::fmt::Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[test]
    fn simple_parse() {
        let iter = parse("{0} and {1}");
        let v: ParserResult<Vec<_>, _> = iter.collect();
        assert_eq!(
            v.unwrap(),
            vec![
                PlaceholderElement::Placeholder(0),
                " and ".into(),
                PlaceholderElement::Placeholder(1),
            ]
        );
    }

    #[test]
    fn simple_interpolate() {
        for sample in samples.iter() {
            let iter = parse::<_, usize>(sample.0);

            let replacements: Vec<Vec<Element<Token>>> = sample
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

    #[test]
    fn named_interpolate() {
        let iter = parse("{start}, {middle} and {end}");

        let replacements: std::collections::HashMap<String, Vec<Element<Token>>> = vec![
            ("start", vec!["Hello"]),
            ("middle", vec!["Middle"]),
            ("end", vec!["World"]),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.iter().map(|&t| t.into()).collect()))
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
        assert_eq!(result, "Hello, Middle and World");
    }

    #[test]
    fn placeholder_parse_err() {
        let iter = parse::<_, usize>("{0} and {end}");
        let v: ParserResult<Vec<_>, _> = iter.collect();
        assert_eq!(v.is_err(), true);
    }

    #[test]
    fn placeholder_date_time() {
        #[derive(Debug)]
        enum Token {
            Year,
            Month,
            Day,
            Hour,
            Minute,
        }

        impl std::fmt::Display for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ch = match self {
                    Self::Year => 'Y',
                    Self::Month => 'M',
                    Self::Day => 'd',
                    Self::Hour => 'H',
                    Self::Minute => 'm',
                };
                f.write_char(ch)
            }
        }

        let date: Vec<Element<Token>> = vec![
            Element::Token(Token::Year),
            "-".into(),
            Element::Token(Token::Month),
            "-".into(),
            Element::Token(Token::Day),
        ];

        let time = vec![
            Element::Token(Token::Hour),
            ":".into(),
            Element::Token(Token::Minute),
        ];

        let placeholder = "{date}, {time}";

        let mut replacements = HashMap::new();
        replacements.insert("date".to_string(), date);
        replacements.insert("time".to_string(), time);

        let iter = parse::<_, String>(placeholder);

        // let s: String = interpolate(iter, replacements).filter_map(|e| {
        //     match e.unwrap() {
        //         Element::Literal(l) => Some(l),
        //         Element::Token(t) => Some(&t.to_string())
        //     }
        // }).collect();
    }
}
