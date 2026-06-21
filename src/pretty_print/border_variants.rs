#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Get supported characters
pub enum BorderVariants {
    #[default]
    /// Unicode characters
    Unicode,
    /// Code page 437
    CodePage437,
    /// Ascii
    Ascii,
}
impl BorderVariants {
    #[must_use]
    #[allow(unreachable_code)]
    /// Determine the encoding for the current environment
    pub fn determine_codec() -> Self {
        #[cfg(feature = "force_unicode")]
        return BorderVariants::Unicode;

        #[cfg(feature = "force_code_page_437")]
        return BorderVariants::Ascii;

        #[cfg(feature = "force_ascii")]
        return BorderVariants::Ascii;

        #[cfg(not(any(
            feature = "force_unicode",
            feature = "force_ascii",
            feature = "force_code_page_437"
        )))]
        match std::env::var("LANG").as_deref() {
            Ok(lang) if lang.contains("UTF") => Self::Unicode,
            _ => Self::Ascii,
        }
    }

    #[must_use]
    /// Get the top left border corner
    pub const fn get_top_left(&self) -> char {
        match self {
            Self::Ascii => '+',
            Self::CodePage437 | Self::Unicode => '╔',
        }
    }
    #[must_use]
    /// Get the top right border corner
    pub const fn get_top_right(&self) -> char {
        match self {
            Self::Ascii => '+',
            Self::CodePage437 | Self::Unicode => '╗',
        }
    }
    #[must_use]
    /// Get the bottom right border corner
    pub const fn get_bottom_right(&self) -> char {
        match self {
            Self::Ascii => '+',
            Self::CodePage437 | Self::Unicode => '╝',
        }
    }
    #[must_use]
    /// Get the bottom left border corner
    pub const fn get_bottom_left(&self) -> char {
        match self {
            Self::Ascii => '+',
            Self::CodePage437 | Self::Unicode => '╚',
        }
    }
    #[must_use]
    /// Get the straight horizontal border
    pub const fn get_horizontal(&self) -> char {
        match self {
            Self::Ascii => '-',
            Self::CodePage437 | Self::Unicode => '═',
        }
    }
    #[must_use]
    /// Get the straight vertical border
    pub const fn get_vertical(&self) -> char {
        match self {
            Self::Ascii => '|',
            Self::CodePage437 | Self::Unicode => '║',
        }
    }
    #[must_use]
    /// Get the straight vertical border with a connection to the left
    pub const fn get_vertical_left(&self) -> char {
        match self {
            Self::Ascii => '<',
            Self::CodePage437 | Self::Unicode => '╣',
        }
    }
    #[must_use]
    /// Get the straight vertical border with a connection to the right
    pub const fn get_vertical_right(&self) -> char {
        match self {
            Self::Ascii => '>',
            Self::CodePage437 | Self::Unicode => '╠',
        }
    }
}
