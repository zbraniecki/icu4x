use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParserError<R> {
    InvalidPlaceholder(R),
    UnknownPlaceholder(String),
    UnclosedPlaceholder,
    UnclosedLiteral,
}

type ParserResult<E, R> = std::result::Result<E, ParserError<R>>;

#[derive(PartialEq, Debug)]
pub enum PlaceholderElement<'s, P> {
    Placeholder(P),
    Literal(&'s str),
}

#[derive(PartialEq, Debug)]
pub enum Element<'s, T> {
    Token(T),
    Literal(&'s str),
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

pub fn parse<P>(
    input: &str,
) -> impl Iterator<Item = ParserResult<PlaceholderElement<'_, P>, <P as FromStr>::Err>> + '_
where
    P: FromStr + std::fmt::Display,
{
    let mut chars = input.chars().enumerate();
    let mut start_idx = 0;
    let mut state = ParserState::Default;

    std::iter::from_fn(move || {
        while let Some((idx, ch)) = chars.next() {
            match state {
                ParserState::Placeholder if ch == '}' => {
                    state = ParserState::Default;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    match (&input[range]).parse() {
                        Ok(ret) => {
                            return Some(Ok(PlaceholderElement::Placeholder(ret)));
                        }
                        Err(err) => {
                            return Some(Err(ParserError::InvalidPlaceholder(err)));
                        }
                    }
                }
                ParserState::QuotedLiteral if ch == '\'' => {
                    state = ParserState::Default;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    if !range.is_empty() {
                        return Some(Ok(PlaceholderElement::Literal(&input[range])));
                    } else {
                        continue;
                    }
                }
                ParserState::Default if ch == '{' => {
                    state = ParserState::Placeholder;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    if !range.is_empty() {
                        return Some(Ok(PlaceholderElement::Literal(&input[range])));
                    } else {
                        continue;
                    }
                }
                ParserState::Default if ch == '\'' => {
                    state = ParserState::QuotedLiteral;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    if start_idx != idx {
                        return Some(Ok(PlaceholderElement::Literal(&input[range])));
                    } else {
                        continue;
                    }
                }
                _ => {}
            }
        }
        match state {
            ParserState::Placeholder => Some(Err(ParserError::UnclosedPlaceholder)),
            ParserState::QuotedLiteral => Some(Err(ParserError::UnclosedLiteral)),
            ParserState::Default => None,
        }
    })
}

// #[derive(PartialEq, Debug, Clone)]
// pub enum Token<'s> {
//     Literal(&'s str),
// }

// impl<'s> std::fmt::Display for Token<'s> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Literal(s) => f.write_str(s),
//         }
//     }
// }

pub trait ReplacementProvider<'s, P, T> {
    fn take_replacement(&mut self, key: &P) -> Option<Option<Element<'s, T>>>;
}

impl<'s, T> ReplacementProvider<'s, usize, T> for Vec<Vec<Element<'s, T>>> {
    fn take_replacement(&mut self, input: &usize) -> Option<Option<Element<'s, T>>> {
        let r = self.get_mut(*input)?;
        if r.is_empty() {
            Some(None)
        } else {
            Some(Some(r.remove(0)))
        }
    }
}

impl<'s, T> ReplacementProvider<'s, String, T> for HashMap<String, Vec<Element<'s, T>>> {
    fn take_replacement(&mut self, input: &String) -> Option<Option<Element<'s, T>>> {
        let r = self.get_mut(input)?;
        if r.is_empty() {
            Some(None)
        } else {
            Some(Some(r.remove(0)))
        }
    }
}

pub fn interpolate<'s, I, R, P, T>(
    mut pattern: I,
    mut replacements: R,
) -> impl Iterator<Item = ParserResult<Element<'s, T>, <P as FromStr>::Err>>
where
    I: Iterator<Item = ParserResult<PlaceholderElement<'s, P>, <P as FromStr>::Err>>,
    P: FromStr + std::fmt::Display,
    R: ReplacementProvider<'s, P, T>,
{
    let mut current_placeholder = None;

    std::iter::from_fn(move || {
        if let Some(ref placeholder) = &mut current_placeholder {
            match replacements.take_replacement(placeholder) {
                Some(Some(e)) => return Some(Ok(e)),
                Some(None) => {
                    current_placeholder = None;
                }
                None => {
                    return Some(Err(ParserError::UnknownPlaceholder(
                        placeholder.to_string(),
                    )))
                }
            }
        }
        if let Some(element) = pattern.next() {
            match element {
                Ok(PlaceholderElement::Literal(l)) => {
                    return Some(Ok(Element::Literal(l)));
                }
                Ok(PlaceholderElement::Placeholder(p)) => match replacements.take_replacement(&p) {
                    Some(Some(e)) => {
                        current_placeholder = Some(p);
                        return Some(Ok(e));
                    }
                    Some(None) => {
                        panic!();
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
                PlaceholderElement::Literal(" and "),
                PlaceholderElement::Placeholder(1),
            ]
        );
    }

    #[test]
    fn simple_interpolate() {
        for sample in samples.iter() {
            let iter = parse(sample.0);

            let replacements: Vec<Vec<Element<Token>>> = sample
                .1
                .iter()
                .map(|r| r.iter().map(|t| Element::Literal(t)).collect())
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
        .map(|(k, v)| {
            (
                k.to_string(),
                v.iter().map(|t| Element::Literal(t)).collect(),
            )
        })
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
        let iter = parse::<usize>("{0} and {end}");
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

        let date: Vec<Element<Token>> = vec![
            Element::Token(Token::Year),
            Element::Literal("-"),
            Element::Token(Token::Month),
            Element::Literal("-"),
            Element::Token(Token::Day),
        ];

        let time = vec![
            Element::Token(Token::Hour),
            Element::Literal(":"),
            Element::Token(Token::Minute),
        ];

        let placeholder = "{date}, {time}";

        let mut replacements = HashMap::new();
        replacements.insert("date".to_string(), date);
        replacements.insert("time".to_string(), time);

        let iter = parse::<String>(placeholder);

        let i = interpolate(iter, replacements);
        println!("{:#?}", i.collect::<Vec<_>>());
    }
}
