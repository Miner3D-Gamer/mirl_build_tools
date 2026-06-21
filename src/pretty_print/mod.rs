mod border_variants;
mod formatting;
mod pretty_print_text;
pub use border_variants::*;
pub use formatting::*;
pub use pretty_print_text::*;

#[must_use]
/// Get the pretty print text struct
pub fn get_nightly_pretty_print() -> PrettyPrintText {
    let mut message = Vec::new();
    message.push("NIGHTLY REQUIRED".to_pretty_print(PrettyPrintAlignment::Middle));
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
    let width = usize::from(terminal_size::terminal_size().map_or(0, |x| x.0.0));

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
