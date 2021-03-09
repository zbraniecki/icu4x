use icu_simple_formatter::*;
use std::fmt::Write;

#[derive(Debug)]
struct Token;

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn iai_parse() {
    let samples = vec![
        ("Foo {0} and {1}", vec![vec!["Hello"], vec!["World"]]),
        ("Foo {1} and {0}", vec![vec!["Hello"], vec!["World"]]),
        (
            "{0}, {1} and {2}",
            vec![vec!["Start"], vec!["Middle"], vec!["End"]],
        ),
        // ("{start}, {midde} and {end}", vec!["Start", "Middle", "End"]),
        ("{0} 'at' {1}", vec![vec!["Hello"], vec!["World"]]),
    ];

    for sample in &samples {
        let _ = parse::<_, usize>(sample.0).count();
    }
}

fn iai_interpolate() {
    let samples = vec![
        ("Foo {0} and {1}", vec![vec!["Hello"], vec!["World"]]),
        ("Foo {1} and {0}", vec![vec!["Hello"], vec!["World"]]),
        (
            "{0}, {1} and {2}",
            vec![vec!["Start"], vec!["Middle"], vec!["End"]],
        ),
        ("{0} 'at' {1}", vec![vec!["Hello"], vec!["World"]]),
    ];

    for sample in &samples {
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
    }
}

fn iai_named_interpolate() {
    let named_samples = vec![(
        "{start}, {midde} and {end}",
        vec![
            ("start", vec!["Start"]),
            ("middle", vec!["Middle"]),
            ("end", vec!["End"]),
        ],
    )];

    for sample in &named_samples {
        let iter = parse::<_, String>(sample.0);

        let replacements: std::collections::HashMap<String, Vec<Element<Token>>> = sample
            .1
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|&t| t.into()).collect()))
            .collect();

        let _ = interpolate(iter, replacements).count();
    }
}

iai::main!(iai_parse, iai_interpolate, iai_named_interpolate);
