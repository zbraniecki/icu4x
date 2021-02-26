use icu_simple_formatter::*;

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

iai::main!(iai_parse);
