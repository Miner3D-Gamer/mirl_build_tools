use crate::toml::GetCargoError;

#[must_use]
/// Given the lock section of a crate, extract the dependencies
pub fn get_dependencies_of_crate_from_lock_section(section: &str) -> Vec<&str> {
    let to_find = "dependencies = ";
    let Some(start) = section.find(to_find) else {
        return Vec::new();
    };
    let Some(end) = section.find(']') else {
        return Vec::new();
    };
    let list = &section[start + 2..end - 1];
    list.split(',')
        .map(|x| {
            let x = x.trim();
            &x[1..x.len() - 1]
        })
        .collect()
}

/// Get the section about a specific crate
#[must_use]
pub fn get_section_about_crate<'a>(file: &'a str, crate_name: &str) -> Option<&'a str> {
    let to_search = format!("[[package]]\nname = \"{crate_name}\"");

    let start = file.find(&to_search)?;
    let end = start + file[(start)..].find("[[package]]").unwrap_or(file.len());

    Some(&file[start..end])
}
/// Given a lock file, get the info about the requested crate
#[must_use]
pub fn get_package_info_from_lock<'a>(lock: &'a str, package_name: &str) -> Option<&'a str> {
    let to_find = format!("name = \"{package_name}\"");
    let start = lock.find(&to_find)?;
    let end = lock[start..]
        .find("[[package]]")
        .unwrap_or(lock.len() - start)
        + start;

    Some(&lock[start..end - 1])
}

/// Try to get the lock contents of the current workspace.
///
/// # Errors
/// [`GetLockError`]
pub fn get_lock_contents() -> Result<String, GetCargoError> {
    let path = find_workspace_lock_file()?;
    Ok(std::fs::read_to_string(path)?)
}

/// Given a path, go upwards until a Cargo.lock is found
///
/// # Errors
/// When the file could not be found
pub fn find_workspace_lock_file() -> Result<std::path::PathBuf, GetCargoError> {
    let path = std::env::var("CARGO_MANIFEST_DIR")?;

    Ok(find_workspace_lock_from_path(std::path::Path::new(&path))?)
}

/// Given a path, go upwards until a Cargo.lock is found
///
/// # Errors
/// When the file could not be found
pub fn find_workspace_lock_from_path(
    start: &std::path::Path,
) -> Result<std::path::PathBuf, std::io::Error> {
    let mut dir = start;

    loop {
        let candidate = dir.join("Cargo.lock");
        if candidate.exists() {
            return Ok(candidate);
        }

        {
            let Some(parent) = dir.parent() else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Cargo.lock could not be found",
                ));
            };
            dir = parent;
        }
    }
}
