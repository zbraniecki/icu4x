// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date;
use crate::error::DateTimeFormatError;
use crate::fields;
use crate::options::{components, length, preferences, DateTimeFormatOptions};
use crate::pattern::runtime;
use crate::pattern::{hour_cycle, runtime::Pattern};
use crate::provider;
use crate::provider::gregory::patterns::PatternFromPatternsV1Marker;
use crate::provider::gregory::PatternFromSkeletonsV1Marker;
use crate::provider::gregory::{DatePatternsV1, DateSkeletonPatternsV1};
use crate::provider::gregory::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker};
use crate::skeleton;
use alloc::borrow::Cow;
use icu_locid::Locale;
use icu_provider::prelude::*;

type Result<T> = core::result::Result<T, DateTimeFormatError>;

/// This function is used to select appropriate pattern from data provider
/// data for the given options and locale.
///
/// It uses a temporary structure `PatternSelector` to lazily load data as needed
/// as it traverses the decision tree based on the provided options.
pub(crate) fn pattern_for_options<'data, D>(
    data_provider: &D,
    locale: &Locale,
    options: &DateTimeFormatOptions,
) -> Result<DataPayload<'data, PatternFromPatternsV1Marker>>
where
    D: DataProvider<'data, DatePatternsV1Marker>
        + DataProvider<'data, DateSkeletonPatternsV1Marker>,
{
    match options {
        DateTimeFormatOptions::Length(bag) => pattern_for_length_bag(data_provider, locale, bag),
        DateTimeFormatOptions::Components(bag) => {
            pattern_for_components_bag(data_provider, locale, bag)
        }
    }
}

/// Determine the appropriate `Pattern` for a given `options::Length` bag.
fn pattern_for_length_bag<'data, D>(
    data_provider: &D,
    locale: &Locale,
    length: &length::Bag,
) -> Result<DataPayload<'data, PatternFromPatternsV1Marker>>
where
    D: DataProvider<'data, DatePatternsV1Marker>,
{
    let patterns = retrieve_patterns(data_provider, locale)?;
    Ok(
        patterns.map_project_with_capture(length, |data, length, _| {
            let pattern = match (length.date, length.time) {
                (None, None) => Pattern::default(),
                (None, Some(time_length)) => {
                    pattern_for_time_length(data, time_length, &length.preferences)
                }
                (Some(date_length), None) => pattern_for_date_length(data, date_length),
                (Some(date_length), Some(time_length)) => {
                    pattern_for_datetime_length(data, date_length, time_length, &length.preferences)
                }
            };
            pattern.into()
        }),
    )
}

fn retrieve_patterns<'data, D>(
    data_provider: &D,
    locale: &Locale,
) -> Result<DataPayload<'data, DatePatternsV1Marker>>
where
    D: DataProvider<'data, DatePatternsV1Marker>,
{
    let patterns_data = data_provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: provider::key::GREGORY_DATE_PATTERNS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(locale.clone().into()),
                },
            },
        })?
        .take_payload()?;
    Ok(patterns_data)
}

fn retrieve_skeletons<'data, D>(
    data_provider: &D,
    locale: &Locale,
) -> Result<DataPayload<'data, DateSkeletonPatternsV1Marker>>
where
    D: DataProvider<'data, DateSkeletonPatternsV1Marker>,
{
    let patterns_data = data_provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: provider::key::GREGORY_DATE_SKELETON_PATTERNS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(locale.clone().into()),
                },
            },
        })?
        .take_payload()?;
    Ok(patterns_data)
}

fn pattern_for_date_length<'data>(
    patterns: DatePatternsV1<'data>,
    length: length::Date,
) -> Pattern<'data> {
    match length {
        length::Date::Full => patterns.date.full,
        length::Date::Long => patterns.date.long,
        length::Date::Medium => patterns.date.medium,
        length::Date::Short => patterns.date.short,
    }
}

fn pattern_for_date_length2<'a, 'data>(
    patterns: &'a DatePatternsV1<'data>,
    length: length::Date,
) -> &'a Pattern<'data> {
    match length {
        length::Date::Full => &patterns.date.full,
        length::Date::Long => &patterns.date.long,
        length::Date::Medium => &patterns.date.medium,
        length::Date::Short => &patterns.date.short,
    }
}

fn pattern_for_time_length<'data>(
    patterns: DatePatternsV1<'data>,
    length: length::Time,
    preferences: &Option<preferences::Bag>,
) -> Pattern<'data> {
    // Determine the coarse hour cycle patterns to use from either the preference bag,
    // or the preferred hour cycle for the locale.
    let time = if let Some(preferences::Bag {
        hour_cycle: Some(hour_cycle_pref),
    }) = preferences
    {
        match hour_cycle_pref {
            preferences::HourCycle::H11 | preferences::HourCycle::H12 => patterns.time_h11_h12,
            preferences::HourCycle::H23 | preferences::HourCycle::H24 => patterns.time_h23_h24,
        }
    } else {
        match patterns.preferred_hour_cycle {
            crate::pattern::CoarseHourCycle::H11H12 => patterns.time_h11_h12,
            crate::pattern::CoarseHourCycle::H23H24 => patterns.time_h23_h24,
        }
    };

    let pattern = match length {
        length::Time::Full => time.full,
        length::Time::Long => time.long,
        length::Time::Medium => time.medium,
        length::Time::Short => time.short,
    };
    // hour_cycle::naively_apply_preferences(&mut pattern, preferences);
    pattern
}

fn pattern_for_time_length2<'a, 'data>(
    patterns: &'a DatePatternsV1<'data>,
    length: length::Time,
    preferences: &Option<preferences::Bag>,
) -> &'a Pattern<'data> {
    // Determine the coarse hour cycle patterns to use from either the preference bag,
    // or the preferred hour cycle for the locale.
    let time = if let Some(preferences::Bag {
        hour_cycle: Some(hour_cycle_pref),
    }) = preferences
    {
        match hour_cycle_pref {
            preferences::HourCycle::H11 | preferences::HourCycle::H12 => &patterns.time_h11_h12,
            preferences::HourCycle::H23 | preferences::HourCycle::H24 => &patterns.time_h23_h24,
        }
    } else {
        match patterns.preferred_hour_cycle {
            crate::pattern::CoarseHourCycle::H11H12 => &patterns.time_h11_h12,
            crate::pattern::CoarseHourCycle::H23H24 => &patterns.time_h23_h24,
        }
    };

    let pattern = match length {
        length::Time::Full => &time.full,
        length::Time::Long => &time.long,
        length::Time::Medium => &time.medium,
        length::Time::Short => &time.short,
    };
    // hour_cycle::naively_apply_preferences(&mut pattern, preferences);
    pattern
}

fn pattern_for_datetime_length<'data>(
    patterns: DatePatternsV1<'data>,
    date_length: length::Date,
    time_length: length::Time,
    preferences: &Option<preferences::Bag>,
) -> Pattern<'data> {
    let pattern = match date_length {
        length::Date::Full => patterns.length_combinations.full.clone(),
        length::Date::Long => patterns.length_combinations.long.clone(),
        length::Date::Medium => patterns.length_combinations.medium.clone(),
        length::Date::Short => patterns.length_combinations.short.clone(),
    };
    let date = pattern_for_date_length2(&patterns, date_length);
    let time = pattern_for_time_length2(&patterns, time_length, preferences);
    pattern.combined(date, time).unwrap().into()
}

/// Determine the appropriate `Pattern` for a given `options::components::Bag`.
fn pattern_for_components_bag<'data, D>(
    data_provider: &D,
    locale: &Locale,
    components: &components::Bag,
) -> Result<DataPayload<'data, PatternFromPatternsV1Marker>>
where
    D: DataProvider<'data, DatePatternsV1Marker>
        + DataProvider<'data, DateSkeletonPatternsV1Marker>,
{
    //XXX: This should lazily load patterns and eagrly skeletons.
    let patterns: DataPayload<'data, DatePatternsV1Marker> =
        retrieve_patterns(data_provider, locale)?;
    Ok(patterns.map_project_with_capture(
        (data_provider, locale, components),
        |data, (data_provider, locale, components), _| {
            let skeletons = retrieve_skeletons(data_provider, locale).unwrap();

            // Not all skeletons are currently supported.
            let requested_fields = components.to_vec_fields();
            let pattern = match skeleton::create_best_pattern_for_fields(
                skeletons.get(),
                data.length_combinations,
                &requested_fields,
                components,
                false, // Prefer the requested fields over the matched pattern.
            ) {
                //XXX: Need to clone to 'static here to escape lifetimes.
                skeleton::BestSkeleton::AllFieldsMatch(pattern)
                | skeleton::BestSkeleton::MissingOrExtraFields(pattern) => pattern.0.into_owned(),
                skeleton::BestSkeleton::NoMatch => Pattern::default(),
            };
            pattern.into()
        },
    ))
}

pub trait DateTimeSymbols {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str>;
}

impl DateTimeSymbols for provider::gregory::DateSymbolsV1 {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str> {
        let widths = match weekday {
            fields::Weekday::Format => &self.weekdays.format,
            fields::Weekday::StandAlone => {
                if let Some(ref widths) = self.weekdays.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        fields::FieldLength::Six => widths
                            .short
                            .as_ref()
                            .or_else(|| widths.abbreviated.as_ref()),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[(day as usize) % 7];
                    } else {
                        return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                    }
                } else {
                    return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                }
            }
            fields::Weekday::Local => unimplemented!(),
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            fields::FieldLength::Six => widths.short.as_ref().unwrap_or(&widths.abbreviated),
            _ => &widths.abbreviated,
        };
        &symbols.0[(day as usize) % 7]
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str> {
        // TODO(#493): Support symbols for non-Gregorian calendars.
        debug_assert!(num < 12);
        let widths = match month {
            fields::Month::Format => &self.months.format,
            fields::Month::StandAlone => {
                if let Some(ref widths) = self.months.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[num];
                    } else {
                        return self.get_symbol_for_month(fields::Month::Format, length, num);
                    }
                } else {
                    return self.get_symbol_for_month(fields::Month::Format, length, num);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        &symbols.0[num]
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str> {
        use fields::{DayPeriod::NoonMidnight, FieldLength};
        let widths = &self.day_periods.format;
        let symbols = match length {
            FieldLength::Wide => &widths.wide,
            FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        match (day_period, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => symbols.midnight.as_ref().unwrap_or(&symbols.am),
            (NoonMidnight, 12, true) => symbols.noon.as_ref().unwrap_or(&symbols.pm),
            (_, hour, _) if hour < 12 => &symbols.am,
            _ => &symbols.pm,
        }
    }
}
