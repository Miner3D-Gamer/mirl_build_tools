//! Mirl requires nightly, this lib makes sure the user knows
//!
//! Q/A(/S):
//! Q: Doesn't this just increase compile times and pollute the target cache?
//! A: That is correct in theory. However, the increased clarity is worth this tiny, almost invisible, increase in resources
//! S: Most crates using this lib have a feature called `no_nightly_check` that disables this crate

#[cfg_attr(all(feature = "strum"), derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "wincode",
    derive(wincode::SchemaRead, wincode::SchemaWrite)
)]
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
trait PrettyPrintConvenience {
    fn to_pretty_print(
        &self,
        alignment: PrettyPrintAlignment,
    ) -> PrettyPrintFormat;
}
impl PrettyPrintConvenience for &'_ str {
    fn to_pretty_print(
        &self,
        alignment: PrettyPrintAlignment,
    ) -> PrettyPrintFormat {
        PrettyPrintFormat::Text(self.to_string(), alignment)
    }
}
#[cfg_attr(all(feature = "strum"), derive(strum::EnumIter))]
#[cfg_attr(all(feature = "enum_ext"), enum_ext::enum_extend)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "wincode",
    derive(wincode::SchemaRead, wincode::SchemaWrite)
)]
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
#[cfg_attr(all(feature = "strum"), derive(strum::EnumIter))]
#[cfg_attr(all(feature = "enum_ext"), enum_ext::enum_extend)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "wincode",
    derive(wincode::SchemaRead, wincode::SchemaWrite)
)]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "wincode",
    derive(wincode::SchemaRead, wincode::SchemaWrite)
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The text holder
pub struct PrettyPrintText {
    /// The lines to pp
    pub lines: Vec<PrettyPrintFormat>,
    /// If there should be a horizontal margin between the text and the border
    pub margin: bool,
    /// If the text should have border
    pub border: bool,
}
impl PrettyPrintText {
    #[must_use]
    /// Get the longest line length
    pub fn get_longest_line_length(&self) -> usize {
        let mut longest = 0;

        for l in &self.lines {
            let length = match l {
                PrettyPrintFormat::Divider => 0,
                PrettyPrintFormat::Text(t, _) => t.chars().count(),
            };
            if length > longest {
                longest = length;
            }
        }
        longest
    }
    #[must_use]
    /// Estimate the size of all characters (outside of border)
    pub fn get_estimated_size(&self, longest: usize) -> usize {
        self.lines
            .iter()
            .map(|x| match x {
                PrettyPrintFormat::Text(x, _) => x.chars().count(),
                PrettyPrintFormat::Divider => longest,
            })
            .sum()
    }
    #[must_use]
    /// Turn the given lines into text
    pub fn to_text(&self, formatting: BorderVariants) -> String {
        let text_length = self.get_longest_line_length();
        let longest = text_length
            + if self.margin {
                2
            } else {
                0
            };
        let mut output = String::with_capacity(
            self.get_estimated_size(longest) + self.lines.len() * 2,
        );
        if self.border {
            output.push(formatting.get_top_left());
            output.push_str(
                &formatting.get_horizontal().to_string().repeat(longest),
            );
            output.push(formatting.get_top_right());
        }
        for line in &self.lines {
            output.push('\n');
            match line {
                PrettyPrintFormat::Divider => {
                    if self.border {
                        output.push(formatting.get_vertical_right());
                    }
                    output.push_str(
                        &formatting
                            .get_horizontal()
                            .to_string()
                            .repeat(longest),
                    );
                    if self.border {
                        output.push(formatting.get_vertical_left());
                    }
                }
                PrettyPrintFormat::Text(text, alignment) => {
                    if self.border {
                        output.push(formatting.get_vertical());
                        if self.margin {
                            output.push(' ');
                        }
                    }
                    match alignment {
                        PrettyPrintAlignment::Front => {
                            output.push_str(text);
                            output.push_str(
                                &" ".repeat(text_length - text.chars().count()),
                            );
                        }
                        PrettyPrintAlignment::End => {
                            output.push_str(
                                &" ".repeat(text_length - text.chars().count()),
                            );
                            output.push_str(text);
                        }
                        PrettyPrintAlignment::Middle => {
                            let c = text_length - text.chars().count();
                            let f = c / 2;
                            let e = if c.is_multiple_of(2) {
                                f
                            } else {
                                f + 1
                            };
                            output.push_str(&" ".repeat(f));
                            output.push_str(text);
                            output.push_str(&" ".repeat(e));
                        }
                    }
                    if self.border {
                        if self.margin {
                            output.push(' ');
                        }
                        output.push(formatting.get_vertical());
                    }
                }
            }
        }
        if self.border {
            output.push('\n');
            output.push(formatting.get_bottom_left());
            output.push_str(
                &formatting.get_horizontal().to_string().repeat(longest),
            );
            output.push(formatting.get_bottom_right());
        }

        output
    }
}
#[must_use]
/// Get the pretty print text struct
pub fn get_nightly_pretty_print() -> PrettyPrintText {
    let mut message = Vec::new();
    message
        .push("NIGHTLY REQUIRED".to_pretty_print(PrettyPrintAlignment::Middle));
    message.push(PrettyPrintFormat::Divider);

    let front = vec![
        "All libs under `Mirl` require nightly to compile.",
        "",
        "To install and use nightly:",
        "",
        "1. Install nightly toolchain:",
        "   `rustup install nightly`",
        "",
        "2. Use nightly for this project (recommended):",
        "   `rustup override set nightly`",
        "",
        "Or use nightly for a single build:",
        "   `cargo +nightly build`",
    ];
    let mut longest = 0;
    for i in front {
        longest = longest.max(i.chars().count());
        message.push(i.to_pretty_print(PrettyPrintAlignment::Front));
    }

    #[cfg(feature = "border")]
    let width =
        usize::from(terminal_size::terminal_size().map_or(0, |x| x.0.0));

    #[cfg(feature = "border")]
    let border = width >= longest + 4;
    #[cfg(not(feature = "border"))]
    let border = false;

    #[cfg(feature = "margin")]
    let margin = width >= longest + 2;
    #[cfg(not(feature = "margin"))]
    let margin = false;

    PrettyPrintText {
        lines: message,
        margin,
        border,
    }
}

/// Print the "nightly required" screen
pub fn print_nightly() {
    eprintln!(
        "{}",
        get_nightly_pretty_print().to_text(BorderVariants::determine_codec())
    );
}

#[test]
fn test_print() {
    ensure_nightly();
    print_nightly();
}

// /// Print the header of the nightly message
// pub fn print_nightly_header() {
//     eprintln!("╔══════════════════════════════════════════════════╗");
//     eprintln!("║                 NIGHTLY REQUIRED                 ║");
//     eprintln!("╠══════════════════════════════════════════════════╣");
// }
// /// Print the description of the nightly message
// fn print_nightly_description() {
//     if let Some(_name) = std::env::var("MIRL_CRATE_NAME").ok()
//         && false
//     {
//         // eprintln!("║ `{}` under the name of `Mirl`                ║");
//     } else {
//         eprintln!("║ This libs under the name of `Mirl`               ║");
//     }
//     eprintln!("║ All libs under `Mirl` require nightly to compile.║");
// }
// /// Print the installation instructions for nightly
// pub fn print_nightly_instructions() {
//     eprintln!("║                                                  ║");
//     eprintln!("║ To install and use nightly:                      ║");
//     eprintln!("║                                                  ║");
//     eprintln!("║ 1. Install nightly toolchain:                    ║");
//     eprintln!("║    `rustup install nightly`                      ║");
//     eprintln!("║                                                  ║");
//     eprintln!("║ 2. Use nightly for this project (recommended):   ║");
//     eprintln!("║    `rustup override set nightly`                 ║");
//     eprintln!("║                                                  ║");
//     eprintln!("║ Or use nightly for a single build:               ║");
//     eprintln!("║    `cargo +nightly build`                        ║");
//     eprintln!("╚══════════════════════════════════════════════════╝");
// }
/// Ensure that the user compiles with nightly. If they don't, give them a nice error message
///
/// # Panics
/// When unable to infer the rust version
pub fn ensure_nightly() {
    // Check if we're using the nightly compiler
    let Some(is_nightly) = version_check::is_feature_flaggable() else {
        eprintln!("Unable to infer rust metadata using `version_check` crate");
        return;
    };

    if !is_nightly {
        print_nightly();

        // Exit with error code
        std::process::exit(1);
    }
    #[cfg(target_os = "linux")]
    detect_linux_visual_backend();

    println!("cargo:rerun-if-changed=build.rs");
}
/// Detect if the linux user uses WAYLAND or X11 (preferring WAYLAND)
pub fn detect_linux_visual_backend() {
    let wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

    if wayland {
        println!("cargo:rustc-cfg=is_wayland");
        println!("cargo:rustc-cfg=visuals_supported");
    } else {
        let x11 = std::env::var("DISPLAY").is_ok();
        if x11 {
            println!("cargo:rustc-cfg=is_x11");
            println!("cargo:rustc-cfg=visuals_supported");
        }
    }

    // Re-run if these variables change
    println!("cargo:rerun-if-env-changed=WAYLAND_DISPLAY");
    println!("cargo:rerun-if-env-changed=DISPLAY");
}
