use crate::pretty_print::{BorderVariants, PrettyPrintAlignment, PrettyPrintFormat};

#[cfg_attr(
    feature = "mirl_derive",
    mirl_derive::derive_all(zerocopy = false, compactly = false,)
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
        let longest = text_length + if self.margin { 2 } else { 0 };
        let mut output =
            String::with_capacity(self.get_estimated_size(longest) + self.lines.len() * 2);
        if self.border {
            output.push(formatting.get_top_left());
            output.push_str(&formatting.get_horizontal().to_string().repeat(longest));
            output.push(formatting.get_top_right());
        }
        for line in &self.lines {
            output.push('\n');
            match line {
                PrettyPrintFormat::Divider => {
                    if self.border {
                        output.push(formatting.get_vertical_right());
                    }
                    output.push_str(&formatting.get_horizontal().to_string().repeat(longest));
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
                            output.push_str(&" ".repeat(text_length - text.chars().count()));
                        }
                        PrettyPrintAlignment::End => {
                            output.push_str(&" ".repeat(text_length - text.chars().count()));
                            output.push_str(text);
                        }
                        PrettyPrintAlignment::Middle => {
                            let c = text_length - text.chars().count();
                            let f = c / 2;
                            let e = if c.is_multiple_of(2) { f } else { f + 1 };
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
            output.push_str(&formatting.get_horizontal().to_string().repeat(longest));
            output.push(formatting.get_bottom_right());
        }

        output
    }
}
