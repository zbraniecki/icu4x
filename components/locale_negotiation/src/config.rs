use icu_locale::Locale;

#[derive(Default, PartialEq, Eq)]
pub enum NegotiationStrategy {
    Filtering,
    #[default]
    Matching,
    BestMatch,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ExtensionHandlingStrategy {
    /// Preserve per-locale extensions from each desired locale
    /// (Windows-style: each locale can have its own extensions)
    PerLocaleExtensions,

    /// Apply global extensions to all results
    /// (MacOS-style: extensions are set system-wide)
    GlobalExtensions,

    /// Both per-locale and global extensions, with per-locale taking precedence
    /// (Hybrid approach)
    #[default]
    HybridExtensions,

    /// Strip all extensions during matching
    StripExtensions,

    /// Require extensions to match exactly during negotiation
    /// (Most restrictive option)
    MatchExtensions,
}

#[derive(Default)]
pub struct NegotiateConfig {
    pub strategy: NegotiationStrategy,
    //pub use_ext_from_requested: bool,
    pub extension_handling: ExtensionHandlingStrategy,
    //pub global_extensions: Option<Extensions>,
    pub default_locale: Option<Locale>,
}
