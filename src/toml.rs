#[must_use]
/// Given a toml, get the crate name
pub fn get_toml_crate_name(toml: &str) -> Option<String> {
    get_field_in_toml(toml, "name")
}
#[must_use]
/// Given a toml, get the crate version
pub fn get_toml_crate_version(toml: &str) -> Option<String> {
    get_field_in_toml(toml, "version")
}
#[must_use]
/// Given a toml, check if it contains a flag or dependency of the given name
pub fn has_flag_or_dependency(toml: &str, name: &str) -> bool {
    toml.contains(&format!("\n{name} =")) | toml.contains(&format!("\n{name}="))
}
#[must_use]
/// Given a toml, check if it contains flag
pub fn has_flag(toml: &str, name: &str) -> bool {
    get_flag_pos(toml, name).is_some()
}
#[must_use]
/// Given a toml, check if it contains a dependency
pub fn has_dependency(toml: &str, name: &str) -> bool {
    get_dependency_pos(toml, name).is_some()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The position values for a found element
pub struct FoundPosition {
    /// The line where it is found
    pub line: usize,
    /// The column where it is found
    pub column: usize,
    /// The line where the item ends
    pub line_end: usize,
    /// The column where the item ends
    pub column_end: usize,
    /// The line + column where it is found in bytes
    pub item_start: usize,
    /// Where in bytes the item ends
    pub item_end: usize,
    /// Where the line ends in bytes
    pub end: usize,
}

#[must_use]
/// Get range of a section of a toml
pub fn toml_get_section(toml: &str, section: &str) -> Option<FoundPosition> {
    let section = format!("[{section}]");
    let mut found_start = None;
    // crate::compile_warning(format!("Finding {section}"));

    let mut byte_idx = 0;

    for (line_idx, line) in toml.split('\n').enumerate() {
        let trimmed = line.trim_start();
        // crate::compile_warning(format!("Section => {line}"));
        if trimmed.starts_with(&section) {
            let c = line.len() - trimmed.len();
            // crate::compile_warning(format!(
            //     "Took Section => {trimmed} idx {byte_idx} text '{}'",
            //     &toml[byte_idx + c..byte_idx + c + 10]
            // ));
            found_start = Some((byte_idx + c, line_idx + 1, c));
            break;
        }
        byte_idx += line.len() + 1;
    }
    let (item_start, line_idx, column) = found_start?;
    let Some(starting_line_end) = toml[item_start..].find('\n') else {
        // crate::compile_warning("Selection return 1");
        return Some(FoundPosition {
            line: line_idx,
            column,
            line_end: line_idx,
            column_end: toml[line_idx..].find(']').unwrap_or(toml.len()),
            end: toml.len(),
            item_start,
            item_end: toml.len(),
        });
    };
    let starting_line_end = starting_line_end + item_start + 1;
    let mut idx = starting_line_end;

    // crate::compile_warning(format!(
    //     "Region until end from start => {}",
    //     &toml[item_start..]
    // ));
    // crate::compile_warning(format!(
    //     "Region until end => {}",
    //     &toml[starting_line_end..]
    // ));

    for (line_end_idx, line) in toml[starting_line_end..].split('\n').enumerate() {
        let trimmed = line.trim_start();
        // crate::compile_warning(format!("Finding end => {line}"));
        if trimmed.starts_with('[') {
            // let c = line.len() - trimmed.len();
            // crate::compile_warning(format!(
            //     "Selection return 2 - start {}, start_line_end {}, region end {}, toml {}, trimmed {}, loops {}",
            //     item_start,
            //     starting_line_end,
            //     idx,
            //     toml.len(),
            //     trimmed,
            //     line_end_idx
            // ));
            return Some(FoundPosition {
                line: line_idx,
                column,
                line_end: line_end_idx + line_idx,
                column_end: 0,
                end: idx,
                item_start,
                item_end: starting_line_end,
            });
        }
        idx += line.len() + 1;
    }

    // crate::compile_warning("Selection return 3");
    Some(FoundPosition {
        line: line_idx,
        column,
        end: toml.len(),
        line_end: toml.matches('\n').count(),
        column_end: toml
            .chars()
            .rev()
            .position(|x| x == '\n')
            .unwrap_or_default(),
        item_start,
        item_end: starting_line_end,
    })
}

#[must_use]
/// Given a toml, check if it contains a miri flag
pub fn get_dependency_pos(toml: &str, name: &str) -> Option<FoundPosition> {
    get_item_pos_in_section(toml, name, "dependencies")
}
#[must_use]
/// Given a toml, check if it contains a miri flag
pub fn get_flag_pos(toml: &str, name: &str) -> Option<FoundPosition> {
    get_item_pos_in_section(toml, name, "features")
}
#[must_use]
/// Given a toml, check if it contains a miri flag
pub fn get_item_pos_in_section(toml: &str, name: &str, section: &str) -> Option<FoundPosition> {
    let feature_range = toml_get_section(toml, section)?;
    let mut idx = feature_range.item_end;

    let mut data = None;
    // crate::compile_warning(format!("Range: {}", &toml[idx..feature_range.end]));

    for (line_idx, line) in toml[idx..feature_range.end].split('\n').enumerate() {
        let trimmed = line.trim_start();

        // crate::compile_warning(format!("Line: {line}"));
        if trimmed.starts_with(name)
            && [' ', '\t', '=']
                .map(|x| unsafe { u8::try_from(u32::from(x)).unwrap_unchecked() })
                .contains(&trimmed.as_bytes()[name.len()])
        {
            let c = line.len() - trimmed.len();
            let start = idx + c;
            data = Some(FoundPosition {
                line: line_idx + feature_range.line + 1,
                column: c,
                item_start: start,
                item_end: trimmed.find('#').unwrap_or_else(|| trimmed.len() + start),
                end: trimmed.len() + start,
                line_end: 0,
                column_end: 0,
            });
            break;
        }

        idx += line.len() + 1;
    }
    // crate::compile_warning("Before data");
    let mut data = data?;
    // crate::compile_warning("Found data");
    let mut idx = data.item_start;

    for (line_idx, line) in toml[idx..feature_range.end].split('\n').enumerate() {
        let line_without_comment = line.split_once('#').map_or(line, |x| x.0);
        if let Some(pos) = line_without_comment.find(']') {
            data.line_end = data.line + line_idx;
            data.column_end = pos + 2;
            data.item_end = idx + pos;
            return Some(data);
        }
        idx += line.len() + 1;
    }

    crate::compile_warning("FIX THIS");
    None
}
#[must_use]
/// Get the value of a field in toml without parsing the whole toml
pub fn get_field_in_toml(toml: &str, field: &str) -> Option<String> {
    let mut offset = 0;
    let equal_idx = loop {
        if offset >= toml.len() {
            return None;
        }
        let key_idx_start = toml[offset..].find(field)? + offset; // Either you properly format you toml or you ain't doing this right.

        let key_line_end = toml[key_idx_start..].find('\n').unwrap_or(toml.len()) + key_idx_start;

        if key_idx_start != 0 {
            // Safety: The previous character will always exist because we are not doing this check at idx 0
            let before = unsafe {
                toml[key_idx_start - 1..key_idx_start]
                    .chars()
                    .next()
                    .unwrap_unchecked()
            };
            if before != ' ' && before != '\n' {
                offset = key_line_end + 1;
                continue;
            }
        }

        let equal_idx = toml[key_idx_start..key_line_end].find('=')? + key_idx_start;

        // Safety: I think this works
        let key_idx_end = if *unsafe { toml.as_bytes().get(equal_idx - 1).unwrap_unchecked() } == 32
        // 32: Ascii code for space
        {
            // Safety: When we know that there is at least one space, which we check for, this cannot cause UB
            key_idx_start + unsafe { toml[key_idx_start..equal_idx].find(' ').unwrap_unchecked() }
        } else {
            crate::compile_warning("This one? IF SO FIX ME");
            equal_idx
        };
        if toml[key_idx_start..key_idx_end].eq(field) {
            break equal_idx;
        }
        offset = key_line_end + 1;
    };
    let equal_idx = equal_idx + offset;

    let start_idx = toml[equal_idx..].find('"')? + 1 + equal_idx;
    let end_idx = toml[start_idx..].find('"')? + start_idx;
    Some(toml[start_idx..end_idx].to_string())
}

/// Errors that might occur when trying to retrieve Cargo.toml
#[derive(Debug)]
pub enum GetCargoError {
    /// An io error happened
    IOError(std::io::Error),
    /// A var error happened
    VarError(std::env::VarError),
}

impl std::fmt::Display for GetCargoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => {
                write!(f, "IO error reading cargo file: {err}")
            }
            Self::VarError(err) => {
                write!(f, "Environment variable error (file not found): {err}")
            }
        }
    }
}

impl std::error::Error for GetCargoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IOError(err) => Some(err),
            Self::VarError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for GetCargoError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<std::env::VarError> for GetCargoError {
    fn from(err: std::env::VarError) -> Self {
        Self::VarError(err)
    }
}
/// Try to get the toml contents of the currently building lib. Not the highest crate but the one who imported this lib.
///
/// # Errors
/// [`GetCargoError`]
pub fn get_toml_contents() -> Result<String, GetCargoError> {
    Ok(std::fs::read_to_string(get_toml_path()?)?)
}
/// Try to get the toml contents of the currently building lib. Not the highest crate but the one who imported this lib.
///
/// # Errors
/// [`std::env::VarError`]
pub fn get_toml_path() -> Result<String, std::env::VarError> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(format!("{manifest_dir}/Cargo.toml"))
}

/// List all features from a list of lines
///
/// This function is gracefully stolen from the `list_features` crate because of their inability to make functions public
pub fn parse_feature_keys_from_lines<I: IntoIterator<Item = String>>(
    lines: I,
) -> std::collections::HashSet<String> {
    let mut in_features = false;
    let mut features = std::collections::HashSet::new();

    for line in lines {
        let stripped = line.split('#').next().unwrap_or("").trim();

        if stripped.starts_with('[') {
            in_features = stripped == "[features]";
            continue;
        }

        if in_features
            && !stripped.is_empty()
            && let Some((key, _)) = stripped.split_once('=')
        {
            let key = key.trim().trim_matches('"');
            if !key.is_empty() {
                features.insert(key.to_string());
            }
        }
    }

    features
}
