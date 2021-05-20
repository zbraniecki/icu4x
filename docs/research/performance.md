# ICU4X Performance

The latest performance measures have been collected using ICU4X 0.2 and ICU4C 69 on a Dell Tower XXXX using Ubuntu 20.10.

| Component   | Test                                   | ICU4X (bincode) | ICU4X (json) | ICU4C (C++) | ICU4C (Rust) | rust_icu  | Standalone Rust Crates |
|-------------|----------------------------------------|-----------------|--------------|-------------|--------------|-----------|------------------------|
| Locale      |                                        |                 |              |             |              |           |                        |
|             | Create Locale from str for 956 locales | 25,389ns        | 33,828ns     | 736,211ns   | 576,181ns    | 819,582ns | 37,286ns               |
|             | Match 956 against en-US                | 2,579ns         | 4,185ns      | 27,331ns    | 10,873ns     | 5,599ns   | 3,209ns                |
|             | Serialize 956 locales                  | 57,115ns        | 113,311ns    | 1,377,886ns | 61,587ns     | 76,043ns  | 61,165ns               |
| PluralRules |                                        |                 |              |             |              |           |                        |
|             | Select                                 | 79ns            | 88ns         | 68,871ns    | 233ns        | 321ns     | 4ns                    |
| DateTime    |                                        |                 |              |             |              |           |                        |
|             | Format                                 | 3,423ns         | 5,378ns      | 2,231,000ns | ?            | 8,047ns   | 32ns                   |


