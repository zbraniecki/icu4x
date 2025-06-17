use criterion::{criterion_group, criterion_main, Criterion};

use icu_locale::locale;
use icu_locale_negotiation::{
    config::{NegotiateConfig, NegotiationStrategy},
    LocaleNegotiator, WeightedLocale,
};

fn negotiate_bench(c: &mut Criterion) {
    let available = vec![
        locale!("en-US"),
        locale!("fr"),
        locale!("de"),
        locale!("en-GB"),
        locale!("it"),
        locale!("pl"),
        locale!("ru"),
        locale!("sr-Cyrl"),
        locale!("sr-Latn"),
        locale!("ja-JP"),
        locale!("he-IL"),
        locale!("de-DE"),
        locale!("de-IT"),
    ];
    let requested = vec![
        WeightedLocale::new(locale!("de"), 1.0),
        WeightedLocale::new(locale!("it"), 0.5),
        WeightedLocale::new(locale!("ru"), 0.3),
    ];

    c.bench_function("negotiate", |b| {
        b.iter(|| {
            let negotiator = LocaleNegotiator::new_common();
            let config = NegotiateConfig {
                strategy: NegotiationStrategy::Filtering,
                ..Default::default()
            };
            let _ = negotiator.filter_and_sort(&available, &requested, config);
        })
    });
}

criterion_group!(benches, negotiate_bench);
criterion_main!(benches);
