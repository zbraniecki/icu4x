// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::langid2::{Language2, LanguageIdentifier2, Region2};
use icu_locid::subtags::{Language, Region};
use icu_locid::LanguageIdentifier;

fn langid_benches(c: &mut Criterion) {
    let data = serde_json::from_str::<fixtures::LocaleList>(include_str!("fixtures/langid.json"))
        .expect("Failed to read a fixture");

    // Overview
    {
        let mut group = c.benchmark_group("langid");

        overview!(group, LanguageIdentifier, &data.canonicalized, "en-US");

        group.finish();
    }

    #[cfg(feature = "bench")]
    {
        use criterion::BenchmarkId;

        // Construct
        {
            let mut group = c.benchmark_group("langid/construct");

            {
                let languages = vec!["pl", "en", "es", "un", "de", "es", "az", "it", "ja", "zh"];

                construct!(group, Language, "language", &languages);

                let new_data: Vec<Vec<u8>> =
                    languages.iter().map(|s| s.as_bytes().to_vec()).collect();

                construct2_utf8!(group, Language2, "language2", &new_data);

                let new_data: Vec<Vec<u16>> = languages
                    .iter()
                    .map(|s| s.encode_utf16().collect())
                    .collect();
                construct2_utf16!(group, Language2, "language2_utf16", &new_data);
            }

            {
                let regions = vec!["PL", "US", "150", "FR", "AT", "419", "AR", "IT", "JP", "CN"];

                construct!(group, Region, "region", &regions);

                let new_data: Vec<Vec<u8>> =
                    regions.iter().map(|s| s.as_bytes().to_vec()).collect();

                construct2_utf8!(group, Region2, "region2", &new_data);

                let new_data: Vec<Vec<u16>> =
                    regions.iter().map(|s| s.encode_utf16().collect()).collect();
                construct2_utf16!(group, Region2, "region2_utf16", &new_data);
            }

            {
                let langids = vec![
                    "pl-PL", "en-US", "es-150", "und-FR", "de-AT", "es-419", "az-AR", "it-IT",
                    "ja-JP", "zh-CN",
                ];

                construct!(group, LanguageIdentifier, "langid", &langids);

                let new_data: Vec<Vec<u8>> =
                    langids.iter().map(|s| s.as_bytes().to_vec()).collect();

                construct2_utf8!(group, LanguageIdentifier2, "langid2", &new_data);

                let new_data: Vec<Vec<u16>> =
                    langids.iter().map(|s| s.encode_utf16().collect()).collect();
                construct2_utf16!(group, LanguageIdentifier2, "langid2_utf16", &new_data);
            }

            group.finish();
        }

        // Stringify
        {
            let mut group = c.benchmark_group("langid/to_string");

            let langids: Vec<LanguageIdentifier> = data
                .canonicalized
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();

            to_string!(group, LanguageIdentifier, "langid", &langids);

            group.finish();
        }

        // Compare
        {
            let mut group = c.benchmark_group("langid/compare");

            // let langids: Vec<LanguageIdentifier> = data
            //     .canonicalized
            //     .iter()
            //     .map(|s| s.parse().unwrap())
            //     .collect();
            // let langids2: Vec<LanguageIdentifier> = data
            //     .canonicalized
            //     .iter()
            //     .map(|s| s.parse().unwrap())
            //     .collect();

            // compare_struct!(group, LanguageIdentifier, "langid", &langids, &langids2);

            // compare_str!(
            //     group,
            //     LanguageIdentifier,
            //     "langid",
            //     &langids,
            //     &data.canonicalized
            // );

            let samples = vec!["pl", "en", "es", "un", "de", "es", "az", "it", "ja", "zh"];

            let languages: Vec<Language2> = samples
                .iter()
                .map(|s| Language2::try_from(s.as_bytes()).unwrap())
                .collect();

            compare_str2!(group, Language2, "language2", &languages, &samples);

            let samples = vec!["PL", "US", "150", "FR", "AT", "419", "AR", "IT", "JP", "CN"];

            let regions: Vec<Region2> = samples
                .iter()
                .map(|s| Region2::try_from(s.as_bytes()).unwrap())
                .collect();

            compare_str2!(group, Region2, "region2", &regions, &samples);

            let samples = vec![
                "pl-PL", "en-US", "es-150", "und-FR", "de-AT", "es-419", "az-AR", "it-IT", "ja-JP",
                "zh-CN",
            ];

            let langids: Vec<LanguageIdentifier> = samples
                .iter()
                .map(|s| LanguageIdentifier::try_from_bytes(s.as_bytes()).unwrap())
                .collect();

            compare_str!(group, LanguageIdentifier, "langid", &langids, &samples);

            let langids: Vec<LanguageIdentifier2> = samples
                .iter()
                .map(|s| LanguageIdentifier2::try_from(s.as_bytes()).unwrap())
                .collect();

            compare_str2!(group, LanguageIdentifier2, "langid2", &langids, &samples);

            group.finish();
        }

        // Canonicalize
        {
            let mut group = c.benchmark_group("langid/canonicalize");

            canonicalize!(group, LanguageIdentifier, "langid", &data.casing);

            group.finish();
        }
    }
}

criterion_group!(benches, langid_benches,);
criterion_main!(benches);
