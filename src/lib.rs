//! Mirl requires nightly, this lib makes sure the user knows
//!
//! ----------
//!
//! How to integrate:
//! ```toml
//! [build-dependencies]
//! mirl_build_tools = "*"
//!
//! [features]
//! miri = ["mirl_build_tools/mirl"]
//! ```
//!
//! And in `build.rs`:
//! ```no_run rust
//! perform_default_tests()
//! ```
//! Or if you don't need custom miri support:
//! ```no_run rust
//! ensure_nightly()
//! ```

/// Libraries that directly expand rust
pub const CRATES_THAT_EXPAND_PRELUDE: &[&str] = &[
    "mirl_extensions",
    "mirl_extensions_math",
    "mirl_extensions_conversion",
    "mirl_extensions_core",
];
/// Libraries that are neat to have
pub const NEAT_CRATES_TO_PRELUDE: &[&str] = &[
    "mirl_buffer",
    "mirl_graphics",
    "mirl_rendering",
    "mirl_system",
    "mirl_windowing",
    "mirl_codec_info",
];

// TODO:
// Using the following, a custom prelude can be defined. Use the listed crates above as candidates to automatically import into every file

// #![feature(prelude_import)]
// #[allow(unreachable_pub)]
// mod custom_prelude {
//     pub use mirl_extensions::*;
//     pub use std::prelude::rust_2024::*;
// }
// #[prelude_import]
// #[allow(unused_imports)]
// use custom_prelude::*;

/// Prettify text for nice visuals in the console
pub mod pretty_print;
use pretty_print::*;

/// Use the configuration options cargo provides
pub mod control;
use control::*;

/// cargo.toml related functions
pub mod toml;
use toml::*;

/// All the info you can extract at build time
pub mod info;

/// Does several checks:
///
/// - If input boolean is set
/// - If the crate has a miri flag and miri is used, the flag must also be set
/// - Adds `CRATE_PRESENT_{crate_name.to_uppercase()}` to the compile time flags so it can be used in `#[cfg(CRATE_PRESENT_{crate_name.to_uppercase()})]`
///
/// TODO:
/// - Add check for the following: if this crate has a feature "X", check if the crate importing it also imports X. If it does, X must also be activated
/// - Add check for the following: If this crate has a feature "X" and a dependency also has X, then the X feature of the dependency should also be included in the X feature of this crate
pub fn setup() {
    // Toml info extraction
    let toml = match get_toml_contents() {
        Ok(val) => val,
        Err(err) => {
            compile_error(format!("Unable to read Cargo.toml: {err}"));
            std::process::exit(1);
        }
    };
    // Crate name present
    let crate_name = get_toml_crate_name(&toml).unwrap_or_else(|| {
        compile_error(
            "Unable to obtain crate name from Cargo.toml (file found and read but name not found inside)",
        );

        std::process::exit(3);
    });
    // Nightly check
    if match detect_nightly(Some(&toml)) {
        Ok(val) => val,
        Err(e) => {
            compile_error(format!("Unable to access lib.rs or main.rs: {e}"));

            std::process::exit(2);
        }
    } {
        // compile_warning(format!(">{crate_name} requires nightly"));
        ensure_nightly();
    }
    // else {
    //     compile_warning(format!("##>{crate_name} does not requires nightly"));
    // }
    // println!("Got name! Here `{}`", crate_name);
    add_rust_compile_time_flag(&format!("CRATE_PRESENT_{}", crate_name.to_uppercase()));

    // Miri check
    let has_miri_flag = had_flag_or_dependency(&toml, "miri");

    if has_miri_flag && !is_executing_using_miri() {
        compile_error(format!(
            "Miri used inside {crate_name} without the miri flag being activated"
        ));
    }
    // Test check
    let has_miri_flag = had_flag_or_dependency(&toml, "test");

    if has_miri_flag && !is_executing_using_miri() {
        compile_error(format!(
            "Test used inside {crate_name} without the test flag being activated"
        ));
    }
}
/// Get the section about a specific crate
#[must_use]
pub fn get_section_about_crate<'a>(file: &'a str, crate_name: &str) -> Option<&'a str> {
    let to_search = format!("[[package]]\nname = \"{crate_name}\"");

    let start = file.find(&to_search)?;
    let end = start
        + file[(start)..]
            .find("[[package]]")
            .unwrap_or(file.len());

    Some(&file[start..end])
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
#[must_use]
/// Check if current execution is running through miri
pub fn is_executing_using_test() -> bool {
    is_env_flag_active("CARGO_CFG_TEST")
}
#[must_use]
/// Check if current execution is running through miri
pub fn is_executing_using_miri() -> bool {
    is_env_flag_active("CARGO_MIRI")
}

#[must_use]
/// Checks if a feature flag is currently activated
///
/// Also returns false when a flag doesn't exist
pub fn is_feature_active<T: Into<String>>(name: T) -> bool {
    let str = name.into().replace('-', "_");

    is_env_flag_active(format!("CARGO_FEATURE_{str}"))
}
#[must_use]
/// Checks if a flag is set to 1 in current env
pub fn is_env_flag_active<T: Into<String>>(name: T) -> bool {
    let str = name.into().to_uppercase();
    let Ok(output) = std::env::var(str) else {
        return false;
    };
    output == "1"
}
#[must_use]
/// Get all active cargo features
pub fn get_all_active_features() -> Vec<String> {
    let mut output = Vec::new();
    for (name, val) in std::env::vars() {
        if name.starts_with("CARGO_FEATURE_") && val == "1" {
            output.push(name);
        }
    }
    output
}

/// Print the "nightly required" screen
pub fn print_nightly_warning() {
    eprintln!(
        "{}",
        get_nightly_pretty_print().to_text(BorderVariants::determine_codec())
    );
}

/// Ensure that the user compiles with nightly. If they don't, give them a nice error message
///
/// When miri is used, assumes that nightly is used regardless of if it is
///
/// # Panics
/// When unable to infer the rust version
pub fn ensure_nightly() {
    // Who uses miri before compiling their project even once anyways?
    #[cfg(not(miri))]
    // Check if we're using the nightly compiler
    let Some(is_nightly) = version_check::is_feature_flaggable() else {
        eprintln!("Unable to infer rust metadata using `version_check` crate");
        return;
    };
    #[cfg(miri)]
    let is_nightly = true;

    if !is_nightly {
        print_nightly_warning();

        // Exit with error code
        std::process::exit(1);
    }
    #[cfg(target_os = "linux")]
    detect_linux_visual_backend();

    println!("cargo:rerun-if-changed=build.rs");
}

/// Checks if the project will require nightly to function
///
/// # Errors
/// When accessing the source code file is not possible
pub fn detect_nightly(toml: Option<&str>) -> Result<bool, std::io::Error> {
    let paths = [get_lib_rs_path(toml), get_main_rs_path(toml)];
    // compile_warning(format!("{:?}",paths));
    for path in paths.into_iter().flatten() {
        // compile_warning(format!("{path}"));
        let file_contents = std::fs::read_to_string(path)?;
        if does_file_use_nightly(&file_contents) {
            return Ok(true);
        }
    }
    Ok(false)
}
#[must_use]
/// Check if a file uses nightly by detecting if a fine uses the feature keyword
pub fn does_file_use_nightly(file: &str) -> bool {
    // TODO: Some idiot could leave a space between the feature and "(", check for that too
    let Some(feature_idx) = file.find("feature(") else {
        return false;
    };
    // eprintln!("{}", &file[feature_idx..feature_idx + 20]);
    let line_start = file.bytes().take(feature_idx).len()
        - file
            .bytes()
            .take(feature_idx)
            .rev()
            .position(|x| x == 10)
            .unwrap_or_default(); // 10 is the Newline symbol

    let line_start_whitespace_offset = file[line_start..]
        .char_indices()
        .skip_while(|x| x.1.is_whitespace())
        .find(|_| true)
        .map(|x| x.0)
        .unwrap_or_default()
        .saturating_sub(1);

    // println!("{line_start} + {line_start_whitespace_offset}");
    let line_start = line_start + line_start_whitespace_offset;

    if file.as_bytes()[line_start] == b'#'
    // line.starts_with('#')
    {
        // TODO: Multiple "#" could be stacked in the same line
        return true;
    }
    let line_end = feature_idx
        + file
            .bytes()
            .skip(feature_idx)
            .position(|x| x == 10)
            .unwrap_or_default(); // 10 is the Newline symbol

    // let line = &file[line_start..line_end];
    // compile_warning("\n");
    // compile_warning("\n");
    // compile_warning(format!(
    //     "> {} --> {:?}\n",
    //     line.replace('\n', "\\n"),
    //     char::from_u32(file.as_bytes()[line_start] as u32).unwrap()
    // ));
    if file[line_end..].contains("feature(") {
        does_file_use_nightly(&file[line_end..])
    } else {
        false
    }
}
/// Get a file relative to the source
#[must_use]
pub fn _get_rs_path(toml: Option<&str>, file: &str) -> Option<String> {
    let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") else {
        return None;
    };

    let lib = std::path::Path::new(&manifest_dir).join(file);

    if lib.exists() {
        return Some(lib.to_string_lossy().to_string());
    }
    if let Some(toml) = toml {
        let path = get_field_in_toml(toml, "path")?;

        let lib = std::path::Path::new(&manifest_dir).join(path);

        if lib.exists() {
            return Some(lib.to_string_lossy().to_string());
        }
        return None;
    }
    None
}
/// Get the path of "lib.rs"
#[must_use]
pub fn get_lib_rs_path(toml: Option<&str>) -> Option<String> {
    _get_rs_path(toml, "src/lib.rs")
}
/// Get the path of "main.rs"
#[must_use]
pub fn get_main_rs_path(toml: Option<&str>) -> Option<String> {
    _get_rs_path(toml, "src/main.rs")
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
/// Print all variables in env
pub fn print_everything_in_env() {
    for (k, v) in std::env::vars() {
        compile_warning(format!("{k}={v}"));
    }
}

#[cfg(test)]
/// Tests for the lib
pub mod test {
    use super::*;

    #[test]
    /// Test if the nightly detection works
    pub fn test_print() {
        ensure_nightly();
        print_nightly_warning();
    }
}
