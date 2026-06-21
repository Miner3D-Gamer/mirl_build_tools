#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// What the pretty print should print
pub enum PrettyPrintFormat {
    /// String and where
    Text(String, PrettyPrintAlignment),
    /// A divider
    Divider,
}
impl Default for PrettyPrintFormat {
    fn default() -> Self {
        Self::Text(String::new(), PrettyPrintAlignment::default())
    }
}
/// Convert self into the pretty print format
pub trait PrettyPrintConvenience {
    /// Convert self into the pretty print format
    fn to_pretty_print(&self, alignment: PrettyPrintAlignment) -> PrettyPrintFormat;
}
impl PrettyPrintConvenience for &'_ str {
    fn to_pretty_print(&self, alignment: PrettyPrintAlignment) -> PrettyPrintFormat {
        PrettyPrintFormat::Text(self.to_string(), alignment)
    }
}
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// Where in line an item should go
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum PrettyPrintAlignment {
    #[default]
    /// Front of the line
    Front,
    /// Middle of line
    Middle,
    /// End of line
    End,
}
impl From<(&'_ str, PrettyPrintAlignment)> for PrettyPrintFormat {
    fn from(value: (&str, PrettyPrintAlignment)) -> Self {
        Self::Text(value.0.to_string(), value.1)
    }
}
