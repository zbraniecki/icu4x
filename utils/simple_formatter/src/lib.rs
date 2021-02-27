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
pub enum Element<'s, P>
where
    P: FromStr + std::fmt::Display,
{
    Placeholder(P),
    Literal(&'s str),
}

#[derive(PartialEq)]
enum ParserState {
    Default,
    Placeholder,
    QuotedLiteral,
}

pub fn parse<P>(
    input: &str,
) -> impl Iterator<Item = ParserResult<Element<'_, P>, <P as FromStr>::Err>> + '_
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
                            return Some(Ok(Element::Placeholder(ret)));
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
                        return Some(Ok(Element::Literal(&input[range])));
                    } else {
                        continue;
                    }
                }
                ParserState::Default if ch == '{' => {
                    state = ParserState::Placeholder;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    if !range.is_empty() {
                        return Some(Ok(Element::Literal(&input[range])));
                    } else {
                        continue;
                    }
                }
                ParserState::Default if ch == '\'' => {
                    state = ParserState::QuotedLiteral;
                    let range = start_idx..idx;
                    start_idx = idx + 1;
                    if start_idx != idx {
                        return Some(Ok(Element::Literal(&input[range])));
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

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'s> {
    Literal(&'s str),
}

impl<'s> std::fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(s) => f.write_str(s),
        }
    }
}

pub trait ReplacementProvider<'s, H> {
    type Iter: Iterator<Item = Token<'s>>;

    fn get_replacement(&self, input: &H) -> Option<Self::Iter>;
}

impl<'s> ReplacementProvider<'s, usize> for Vec<Vec<Token<'s>>> {
    type Iter = <Vec<Token<'s>> as IntoIterator>::IntoIter;

    fn get_replacement(&self, input: &usize) -> Option<Self::Iter> {
        self.get(*input).map(|r| r.clone().into_iter())
    }
}

impl<'s> ReplacementProvider<'s, String> for HashMap<String, Vec<Token<'s>>> {
    type Iter = <Vec<Token<'s>> as IntoIterator>::IntoIter;

    fn get_replacement(&self, input: &String) -> Option<Self::Iter> {
        self.get(input).map(|r| r.clone().into_iter())
    }
}

pub fn interpolate<'s, P, F, H>(
    mut pattern: P,
    replacements: F,
) -> impl Iterator<Item = ParserResult<Token<'s>, <H as FromStr>::Err>>
where
    P: Iterator<Item = ParserResult<Element<'s, H>, <H as FromStr>::Err>>,
    H: FromStr + std::fmt::Display,
    F: ReplacementProvider<'s, H>,
{
    let mut current_placeholder: Option<F::Iter> = None;

    std::iter::from_fn(move || {
        if let Some(ref mut placeholder) = &mut current_placeholder {
            if let Some(v) = placeholder.next() {
                return Some(Ok(v));
            } else {
                current_placeholder = None;
            }
        }
        if let Some(element) = pattern.next() {
            match element {
                Ok(Element::Literal(v)) => {
                    return Some(Ok(Token::Literal(v)));
                }
                Ok(Element::Placeholder(p)) => {
                    if let Some(mut r) = replacements.get_replacement(&p) {
                        let ret = r.next();
                        match ret {
                            Some(ret) => {
                                current_placeholder = Some(r);
                                return Some(Ok(ret));
                            }
                            None => return None,
                        }
                    } else {
                        return Some(Err(ParserError::UnknownPlaceholder(p.to_string())));
                    }
                }
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

    #[test]
    fn simple_parse() {
        let iter = parse("{0} and {1}");
        let v: ParserResult<Vec<_>, _> = iter.collect();
        assert_eq!(
            v.unwrap(),
            vec![
                Element::Placeholder(0),
                Element::Literal(" and "),
                Element::Placeholder(1),
            ]
        );
    }

    #[test]
    fn simple_interpolate() {
        for sample in samples.iter() {
            let iter = parse(sample.0);

            let replacements: Vec<Vec<Token>> = sample
                .1
                .iter()
                .map(|r| r.iter().map(|t| Token::Literal(t)).collect())
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

        let replacements: std::collections::HashMap<String, Vec<Token>> =
            vec![
                ("start", vec!["Hello"]),
                ("middle", vec!["Middle"]),
                ("end", vec!["World"])
            ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|t| Token::Literal(t)).collect()))
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
        assert_eq!(
            v.is_err(),
            true
        );
    }

}
