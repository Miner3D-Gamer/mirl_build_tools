/// Info about a prelude in a function
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CratePrelude {
    /// Crate name
    pub name: &'static str,
    /// Overwrite locations
    pub overwrite: Option<&'static [&'static str]>,
}
impl CratePrelude {
    #[must_use]
    /// Create a new function without any overwrites
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            overwrite: None,
        }
    }
    #[must_use]
    /// Create a new function with overwrites
    pub const fn with_overwrite(name: &'static str, overwrite: &'static [&'static str]) -> Self {
        Self {
            name,
            overwrite: Some(overwrite),
        }
    }
}

/// Libraries that directly expand rust
pub const CRATES_THAT_EXPAND_PRELUDE: &[&CratePrelude] = &[
    &CratePrelude::with_overwrite("mirl_extensions", &["mirl_extensions::*"]),
    &CratePrelude::with_overwrite("mirl_extensions_math", &["mirl_extensions_math::*"]),
    &CratePrelude::with_overwrite(
        "mirl_extensions_conversion",
        &["mirl_extensions_conversion::*"],
    ),
    &CratePrelude::with_overwrite("mirl_extensions_core", &["mirl_extensions_core::*"]),
    &CratePrelude::with_overwrite("itertools", &["itertools::Itertools"]),
    &CratePrelude::with_overwrite("serde", &["serde::Deserialize", "serde::Serialize"]),
    &CratePrelude::with_overwrite("smallvec", &["smallvec::SmallVec"]),
    &CratePrelude::with_overwrite("regex", &["regex::Regex"]),
    &CratePrelude::with_overwrite("async_trait", &["async_trait::async_trait"]),
    &CratePrelude::with_overwrite(
        "zeroize",
        &[
            "zeroize::DefaultIsZeroes",
            "zeroize::TryZeroize",
            "zeroize::Zeroize",
            "zeroize::ZeroizeOnDrop",
        ],
    ),
    &CratePrelude::with_overwrite(
        "strum",
        &[
            "strum::EnumCount",
            "strum::EnumMessage",
            "strum::EnumProperty",
            "strum::IntoDiscriminant",
            "strum::IntoEnumIterator",
            "strum::VariantArray",
            "strum::VariantIterator",
            "strum::VariantMetadata",
            "strum::VariantNames",
        ],
    ),
    &CratePrelude::with_overwrite(
        "zerocopy",
        &[
            "zerocopy::FromBytes",
            "zerocopy::FromZeros",
            "zerocopy::Immutable",
            "zerocopy::IntoBytes",
            "zerocopy::KnownLayout",
            "zerocopy::SplitAt",
            "zerocopy::TryFromBytes",
            "zerocopy::Unaligned",
        ],
    ),
    &CratePrelude::with_overwrite(
        "heck",
        &[
            "heck::ToKebabCase",
            "heck::ToLowerCamelCase",
            "heck::ToPascalCase",
            "heck::ToShoutyKebabCase",
            "heck::ToShoutySnakeCase",
            "heck::ToShoutySnekCase",
            "heck::ToSnakeCase",
            "heck::ToSnekCase",
            "heck::ToTitleCase",
            "heck::ToTrainCase",
            "heck::ToUpperCamelCase",
        ],
    ),
    &CratePrelude::new("bitint"),
];
/// Libraries that are neat to have
pub const NEAT_CRATES_TO_PRELUDE: &[&CratePrelude] = &[
    &CratePrelude::new("mirl_buffer"),
    &CratePrelude::new("mirl_graphics"),
    &CratePrelude::new("mirl_rendering"),
    &CratePrelude::new("mirl_system"),
    &CratePrelude::new("mirl_windowing"),
    &CratePrelude::new("mirl_codec_info"),
    &CratePrelude::with_overwrite("indexmap", &["indexmap::IndexMap", "indexmap::IndexSet"]),
    &CratePrelude::with_overwrite("thiserror", &["thiserror::Error"]),
    &CratePrelude::with_overwrite("serde_json", &["serde_json::Value"]),
];
