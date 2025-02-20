#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-08-preview")]
pub mod package_2022_08_preview;
#[cfg(all(feature = "package-2022-08-preview", not(feature = "no-default-tag")))]
pub use package_2022_08_preview::*;
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "no-default-tag")))]
pub use package_2021_11::*;
#[cfg(feature = "package-2018-10")]
pub mod package_2018_10;
#[cfg(all(feature = "package-2018-10", not(feature = "no-default-tag")))]
pub use package_2018_10::*;
