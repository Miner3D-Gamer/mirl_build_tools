#[must_use]
/// Given a toml, get the crate name
pub fn get_toml_crate_name(toml: &str) -> Option<String> {
    find_field_in_toml(toml, "name")
}
#[must_use]
/// Given a toml, get the crate version
pub fn get_toml_crate_version(toml: &str) -> Option<String> {
    find_field_in_toml(toml, "version")
}
#[must_use]
/// Given a toml, check if it contains a miri flag
pub fn has_miri_flag(toml: &str) -> bool {
    toml.contains("\nmiri =") | toml.contains("\nmiri=")
}
#[must_use]
/// Get the value of a field in toml without parsing the whole toml
pub fn find_field_in_toml(toml: &str, field: &str) -> Option<String> {
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
            println!("This one?");
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
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let cargo_toml_path = format!("{manifest_dir}/Cargo.toml");
    Ok(std::fs::read_to_string(cargo_toml_path)?)
}
