use std::str::FromStr;

#[derive(Debug)]
pub enum ParserError<R> {
    Placeholder(R),
    UnclosedPlaceholder,
}

type ParserResult<E, R> = std::result::Result<E, ParserError<R>>;

#[derive(PartialEq, Debug)]
pub enum Element<'s, P>
where
    P: FromStr,
{
    Placeholder(P),
    Literal(&'s str),
}

pub fn parse<P>(
    input: &str,
) -> impl Iterator<Item = ParserResult<Element<'_, P>, <P as FromStr>::Err>> + '_
where
    P: FromStr,
{
    let mut chars = input.chars().enumerate();
    let mut start_idx = 0;
    let mut in_placeholder = false;

    std::iter::from_fn(move || {
        while let Some((idx, ch)) = chars.next() {
            if in_placeholder {
                let ret = &input[idx..idx + 1];
                chars.next();
                in_placeholder = false;
                start_idx = idx + 2;
                match ret.parse() {
                    Ok(ret) => {
                        return Some(Ok(Element::Placeholder(ret)));
                    },
                    Err(err) => {
                        return Some(Err(ParserError::Placeholder(err)));
                    }
                }
            } else if ch == '{' {
                in_placeholder = true;
                if start_idx != idx {
                    return Some(Ok(Element::Literal(&input[start_idx..idx])));
                } else {
                    continue;
                }
            }
        }
        if in_placeholder {
            return Some(Err(ParserError::UnclosedPlaceholder));
        }
        None
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

pub trait ReplacementProvider<'s, H>
where
    H: FromStr,
{
    type Iter: Iterator<Item = Token<'s>>;

    fn get_replacement(&self, input: H) -> Self::Iter;
}

impl<'s> ReplacementProvider<'s, usize> for Vec<Vec<Token<'s>>> {
    type Iter = <Vec<Token<'s>> as IntoIterator>::IntoIter;

    fn get_replacement(&self, input: usize) -> Self::Iter {
        self.get(input).unwrap().clone().into_iter()
    }
}

pub fn interpolate<'s, P, F, H>(
    mut pattern: P,
    replacements: F,
) -> impl Iterator<Item = ParserResult<Token<'s>, <H as FromStr>::Err>>
where
    P: Iterator<Item = ParserResult<Element<'s, H>, <H as FromStr>::Err>>,
    H: FromStr,
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
                    let mut r = replacements.get_replacement(p);
                    let ret = r.next();
                    match ret {
                        Some(ret) => {
                            current_placeholder = Some(r);
                            return Some(Ok(ret));
                        }
                        None => return None,
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
        (
            "{0} 'at' {1}",
            &[&["Hello"], &["World"]],
            "Hello 'at' World",
        ),
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
            let i = interpolate(iter, replacements);
            let result = i
                .filter_map(|t| {
                    if let Ok(t) = t {
                        Some(t.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("");
            assert_eq!(result, sample.2);
        }
    }
}
