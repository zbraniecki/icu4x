use icu_simple_formatter::*;
use std::fmt::Write;

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
        let _ = parse::<usize>(sample.0).count();
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
        let iter = parse::<usize>(sample.0);

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
        let iter = parse::<String>(sample.0);

        let replacements: std::collections::HashMap<String, Vec<Token>> = sample
            .1
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|t| Token::Literal(t)).collect()))
            .collect();

        let _ = interpolate(iter, replacements).count();
    }
}

iai::main!(iai_parse, iai_interpolate, iai_named_interpolate);
