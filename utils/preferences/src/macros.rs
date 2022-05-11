#[macro_export]
macro_rules! preferences {
    ($name:ident, $resolved_name:ident, {$($key:ident, $rename:tt => $pref:ty, $resolved:ty, $ue:expr),*}) => (
        #[derive(Default, Debug, Clone, PartialEq, Copy)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[non_exhaustive]
        pub struct $name {
            $(
                #[cfg_attr(feature = "serde", serde(rename = $rename))]
                pub $key: $pref,
            )*
        }

        #[derive(Debug, Clone, PartialEq, Copy)]
        pub(crate) struct $resolved_name {
            $(
                $key: $resolved,
            )*
        }

        impl TryFrom<(&$name, &icu_locid::Locale, &$resolved_name)> for $resolved_name {
            type Error = ();

            fn try_from(input: (&$name, &icu_locid::Locale, &$resolved_name)) -> Result<Self, Self::Error> {
                let (prefs, locale, defaults) = input;
                let keywords = &locale.extensions.unicode.keywords;

                $(
                    let mut $key = prefs.$key;
                    if $key.is_none() {
                        $key = $ue.and_then(
                            |ue_key| keywords.get(&ue_key).map(core::convert::TryInto::try_into)
                        ).transpose()?;
                    }
                )*
                Ok($resolved_name {
                    $(
                        $key: $key.unwrap_or(defaults.$key),
                    )*
                })
            }
        }
    )
}
