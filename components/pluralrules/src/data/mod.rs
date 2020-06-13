#[cfg(not(feature = "serde"))]
mod inline;
#[cfg(not(feature = "serde"))]
pub use inline::*;
#[cfg(feature = "serde")]
pub mod json;
#[cfg(feature = "json")]
pub use json::*;
#[cfg(feature = "binary")]
#[cfg(not(feature = "json"))]
pub mod bin;
#[cfg(feature = "binary")]
#[cfg(not(feature = "json"))]
pub use bin::*;
