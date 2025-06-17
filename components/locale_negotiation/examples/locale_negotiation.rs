use icu_locale::locale;
pub use icu_locale_negotiation::LocaleNegotiator;
use icu_locale_negotiation::{
    config::{NegotiateConfig, NegotiationStrategy},
    WeightedLocale,
};

fn main() {
    let available = vec![
        locale!("en-US"),
        locale!("fr-FR"),
        locale!("it-IT"),
        locale!("ja-JP"),
    ];
    let requested = vec![
        WeightedLocale::new(locale!("ja-JP"), 1.0),
        WeightedLocale::new(locale!("it-IT"), 0.5),
    ];
    let negotiator = LocaleNegotiator::new_common();
    let config = NegotiateConfig {
        strategy: NegotiationStrategy::Filtering,
        ..Default::default()
    };
    let result = negotiator.filter_and_sort(&available, &requested, config);
    println!("{:#?}", result);
}
