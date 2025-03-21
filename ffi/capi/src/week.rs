// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::date::ffi::Weekday;
    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{errors::ffi::DataError, locale_core::ffi::Locale};

    #[diplomat::rust_link(icu::calendar::week::RelativeUnit, Enum)]
    #[diplomat::enum_convert(icu_calendar::week::RelativeUnit)]
    pub enum WeekRelativeUnit {
        Previous,
        Current,
        Next,
    }

    #[diplomat::rust_link(icu::calendar::week::WeekOf, Struct)]
    #[diplomat::out]
    pub struct WeekOf {
        pub week: u8,
        pub unit: WeekRelativeUnit,
    }
    /// A Week calculator, useful to be passed in to `week_of_year()` on Date and DateTime types
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::calendar::week::WeekCalculator, Struct)]
    pub struct WeekCalculator(pub icu_calendar::week::WeekCalculator);

    impl WeekCalculator {
        /// Creates a new [`WeekCalculator`] from locale data using compiled data.
        #[diplomat::rust_link(icu::calendar::week::WeekCalculator::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create(locale: &Locale) -> Result<Box<WeekCalculator>, DataError> {
            let prefs = (&locale.0).into();

            Ok(Box::new(WeekCalculator(
                icu_calendar::week::WeekCalculator::try_new(prefs)?,
            )))
        }
        /// Creates a new [`WeekCalculator`] from locale data using a particular data source.
        #[diplomat::rust_link(icu::calendar::week::WeekCalculator::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<WeekCalculator>, DataError> {
            let prefs = (&locale.0).into();

            Ok(Box::new(WeekCalculator(
                icu_calendar::week::WeekCalculator::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                )?,
            )))
        }
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::first_weekday,
            StructField,
            compact
        )]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::min_week_days,
            StructField,
            compact
        )]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_first_day_of_week_and_min_week_days(
            first_weekday: Weekday,
            min_week_days: u8,
        ) -> Box<WeekCalculator> {
            let mut calculator = icu_calendar::week::WeekCalculator::default();
            calculator.first_weekday = first_weekday.into();
            calculator.min_week_days = min_week_days;
            Box::new(WeekCalculator(calculator))
        }

        /// Returns the weekday that starts the week for this object's locale
        #[diplomat::rust_link(icu::calendar::week::WeekCalculator::first_weekday, StructField)]
        #[diplomat::attr(auto, getter)]
        pub fn first_weekday(&self) -> Weekday {
            self.0.first_weekday.into()
        }
        /// The minimum number of days overlapping a year required for a week to be
        /// considered part of that year
        #[diplomat::rust_link(icu::calendar::week::WeekCalculator::min_week_days, StructField)]
        #[diplomat::attr(auto, getter)]
        pub fn min_week_days(&self) -> u8 {
            self.0.min_week_days
        }

        #[diplomat::rust_link(icu::calendar::week::WeekCalculator::weekend, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn weekend(&self) -> WeekendContainsDay {
            let mut contains = WeekendContainsDay::default();
            for day in self.0.weekend() {
                match day {
                    icu_calendar::types::Weekday::Monday => contains.monday = true,
                    icu_calendar::types::Weekday::Tuesday => contains.tuesday = true,
                    icu_calendar::types::Weekday::Wednesday => contains.wednesday = true,
                    icu_calendar::types::Weekday::Thursday => contains.thursday = true,
                    icu_calendar::types::Weekday::Friday => contains.friday = true,
                    icu_calendar::types::Weekday::Saturday => contains.saturday = true,
                    icu_calendar::types::Weekday::Sunday => contains.sunday = true,
                }
            }
            contains
        }
    }

    /// Documents which days of the week are considered to be a part of the weekend
    #[diplomat::rust_link(icu::calendar::week::WeekCalculator::weekend, FnInStruct)]
    #[derive(Default)]
    pub struct WeekendContainsDay {
        pub monday: bool,
        pub tuesday: bool,
        pub wednesday: bool,
        pub thursday: bool,
        pub friday: bool,
        pub saturday: bool,
        pub sunday: bool,
    }
}

impl From<icu_calendar::week::WeekOf> for ffi::WeekOf {
    fn from(other: icu_calendar::week::WeekOf) -> Self {
        ffi::WeekOf {
            week: other.week,
            unit: other.unit.into(),
        }
    }
}
