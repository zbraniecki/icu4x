use icu_datetime::pattern::parser::Parser;

static SAMPLES: &[&str] = &[
    "dd/MM/y",
    "dd/MM",
    "d MMM",
    "d MMM y",
    "MMMM y",
    "d MMMM",
    "HH:mm:ss",
    "HH:mm",
    "y",
    "mm:ss",
    "h:mm:ss",
    "E, h:mm",
    "E, h:mm:ss",
    "E d",
    "E h:mm a",
    "y ",
    "MMM y ",
    "dd/MM",
    "E, dd/MM",
    "LLL",
    "E, d MMM y",
    "E, dd/MM/y",
    "y年M月d日",
    "y年M月d日EEEE",
    "d בMMM y",
    "H นาฬิกา mm นาที ss วินาที",
    "H時mm分ss秒",
];

fn iai_parse_pattern() {
    for sample in SAMPLES {
        let mut p = Parser::new(sample);
        while let Ok(Some(_)) = p.next() {}
    }
}

iai::main!(iai_parse_pattern);
