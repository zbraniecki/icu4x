use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Element<'s, P>
where
    P: FromStr,
{
    Placeholder(P),
    Literal(&'s str),
}

pub fn parse<P>(input: &str) -> impl Iterator<Item = Element<'_, P>> + '_
where
    P: FromStr,
    <P as FromStr>::Err: std::fmt::Debug,
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
                return Some(Element::Placeholder(ret.parse().unwrap()));
            }
            if ch == '{' {
                in_placeholder = true;
                if start_idx != idx {
                    let r: &str = &input[start_idx..idx];
                    let ret = Some(Element::Literal(r));
                    return ret;
                } else {
                    continue;
                }
            }
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

pub fn interpolate<'s, P, F, H>(mut pattern: P, replacements: F) -> impl Iterator<Item = Token<'s>>
where
    P: Iterator<Item = Element<'s, H>>,
    H: FromStr,
    F: ReplacementProvider<'s, H>,
{
    let mut current_placeholder: Option<F::Iter> = None;

    std::iter::from_fn(move || {
        if let Some(ref mut placeholder) = &mut current_placeholder {
            if let Some(v) = placeholder.next() {
                return Some(v);
            } else {
                current_placeholder = None;
            }
        }
        if let Some(element) = pattern.next() {
            match element {
                Element::Literal(v) => {
                    return Some(Token::Literal(v));
                }
                Element::Placeholder(p) => {
                    let mut r = replacements.get_replacement(p);
                    let ret = r.next();
                    current_placeholder = Some(r);
                    return ret;
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
        let v: Vec<Element<usize>> = iter.collect();
        assert_eq!(
            v,
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
            let result = i.map(|t| t.to_string()).collect::<Vec<_>>().join("");
            assert_eq!(result, sample.2);
        }
    }
}
