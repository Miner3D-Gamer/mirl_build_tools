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

/// Use the configuration options cargo provides
pub mod control;
/// Prettify text for nice visuals in the console
pub mod pretty_print;

use control::*;

/// cargo.toml related functions
pub mod toml;
use toml::*;

/// All the info you can extract at build time
pub mod info;

/// Detect/Modify feature flags
pub mod features;
use features::*;
/// cargo.lock related functions
pub mod lock;
/// Nightly related functions
pub mod nightly;
use nightly::*;

/// Cargo metadata related functions
pub mod metadata;

/// Expand the functionality of rust using the creation of custom libs
pub mod prelude_features;
/// Settings for the setup function of this lib
pub mod settings;

pub use settings::SetupSettings;
use settings::*;

/// Does several checks:
///
/// - If input boolean is set
/// - If the crate has a miri flag and miri is used, the flag must also be set
/// - Adds `CRATE_PRESENT_{crate_name.to_uppercase()}` to the compile time flags so it can be used in `#[cfg(CRATE_PRESENT_{crate_name.to_uppercase()})]`
///
/// TODO:
/// - Add check for the following: if this crate has a feature "X", check if the crate importing it also imports X. If it does, X must also be activated
/// - Add check for the following: If this crate has a feature "X" and a dependency also has X, then the X feature of the dependency should also be included in the X feature of this crate
pub fn setup<T: AsRef<SetupSettings>>(settings: T) {
    let settings = settings.as_ref();
    rerun_if_build_rs_changes();

    // Toml info extraction
    let cargo_path = match get_toml_path() {
        Ok(val) => val,
        Err(err) => {
            compile_error(format!("Unable to find Cargo.toml: {err}"));
        }
    };
    rerun_if_file_changes(&cargo_path);
    let toml = match std::fs::read_to_string(&cargo_path) {
        Ok(val) => val,
        Err(err) => {
            compile_error(format!("Unable to read Cargo.toml: {err}"));
        }
    };
    // Crate name present
    let crate_name = get_toml_crate_name(&toml).unwrap_or_else(|| {
        compile_error(
            "Unable to obtain crate name from Cargo.toml (file found and read but name not found inside)",
        );

    });
    // Nightly check
    handle_nightly(settings.nightly, Some(&toml));
    // else {
    //     compile_warning(format!("##>{crate_name} does not requires nightly"));
    // }
    // println!("Got name! Here `{}`", crate_name);

    if settings.set_crate_present_cfg {
        add_rust_compile_time_flag(&format!(
            "CRATE_PRESENT_{}",
            crate_name.to_uppercase().replace('-', "_")
        ));
    }

    handle_flag_check(settings.flag_condition_check, &toml, &crate_name);

    handle_dependency_check(&settings.dependency_check, &crate_name, &cargo_path, &toml);
}
/// Handle the dependency check
pub fn handle_dependency_check(
    settings: &DependencyCheck,
    crate_name: &str,
    toml_path: &str,
    toml: &str,
) {
    match settings {
        DependencyCheck::DoNotCheck => {}
        DependencyCheck::Check(settings) => {
            match crate::metadata::do_dep_feature_check_for_current(crate_name, settings) {
                Ok(missing) => {
                    if !missing.flags.is_empty() {
                        // compile_warning(format!("#> {missing:#?}"));
                        compile_warning(format!(
                            "This crate and its dependencies share flag names which aren't activated when the flags of this crate are.\nIn the file \"{toml_path}\", following flags are missing:",
                        ));

                        let cwd = std::env::current_dir()
                            .unwrap_or_else(|e| compile_error(format!("Unable to obtain cwd: {e}")))
                            .to_string_lossy()
                            .into_owned();

                        let truncated_toml_path = if toml_path.starts_with(&cwd) && false {
                            &toml_path[cwd.len()..]
                        } else {
                            toml_path
                        };

                        let features = missing.parent_features;
                        let mut flags: Vec<(String, Vec<(String, bool)>)> =
                            missing.flags.into_iter().collect();
                        flags.sort();

                        let mut output = Vec::new();
                        for (flag, dep) in flags {
                            let mut final_flags = features
                                .get_flag(&flag)
                                .map(|x| x.activating.clone())
                                .unwrap_or_default()
                                .iter()
                                .map(|x| format!("\"{x}\""))
                                .collect::<Vec<String>>();
                            final_flags.extend(dep.iter().map(|(flag_dep, is_weak)| {
                                format!("\"{flag_dep}{}/{flag}\"", if *is_weak { "?" } else { "" })
                            }));

                            let pos = get_flag_pos(toml, &flag).unwrap_or_else(|| compile_error(format!(
                                    "Feature flag was detected but couldn't be found in \"{toml_path}\": {flag}"
                                )));
                            output.push((pos, flag, final_flags));
                            // compile_warning(format!(
                            //     "From \"{truncated_toml_path}:{}:{}\" to {}:{} ({}:{}) => {flag} = [{}]",
                            //     pos.line,
                            //     pos.column,
                            //     pos.line_end,
                            //     pos.column_end,
                            //     pos.item_start,pos.item_end,
                            //     final_flags.join(", ")
                            // ));
                        }
                        output.sort_by_key(|x| x.0.line);
                        for (pos, flag, final_flags) in output {
                            let path = format!(
                                "{truncated_toml_path}:{}:{}-{}",
                                pos.line,
                                pos.column,
                                if pos.line == pos.line_end {
                                    format!("{}", pos.column_end)
                                } else {
                                    format!("{}:{}", pos.line_end, pos.column_end)
                                }
                            );
                            compile_warning(format!(
                                "From \"{path}\" ({}:{}) => {flag} = [{}]",
                                pos.item_start,
                                pos.item_end,
                                final_flags.join(", ")
                            ));
                        }
                    }
                }
                Err(err) => {
                    compile_error(format!("# When trying to do a dependency check> {err:?}"))
                }
            }
        }
    }
}
/// Handle the known flag checking
pub fn handle_flag_check(settings: FlagConditionCheck, toml: &str, name: &str) {
    // Miri check
    if settings.miri && is_flag_checked_under_condition(toml, "miri", is_executing_using_miri) {
        compile_error(format!(
            "Miri used inside {name} without the miri flag being activated"
        ));
    }
    // Test check
    if settings.test && is_flag_checked_under_condition(toml, "test", is_executing_using_test) {
        compile_error(format!(
            "Test used inside {name} without the test flag being activated"
        ));
    }
}

/// Handle the known flag checking for as single item
pub fn is_flag_checked_under_condition(
    toml: &str,
    name: &str,
    condition: impl Fn() -> bool,
) -> bool {
    has_flag_or_dependency(toml, name) && condition() && !is_feature_active(name)
}

/// Handle nightly behavior with the given settings
pub fn handle_nightly(settings: NightlySettings, toml: Option<&str>) {
    let nightly = match settings {
        NightlySettings::DetectAutomatically => match detect_nightly(toml) {
            Ok(val) => val,
            Err(e) => {
                compile_error(format!("Unable to access lib.rs or main.rs: {e}"));
            }
        },
        NightlySettings::OverwriteWithNightly => true,
        NightlySettings::OverwriteWithStable => false,
    };
    add_rust_compile_time_flag("IS_NIGHTLY");
    // Nightly check
    if nightly {
        add_rust_compile_time_flag("IS_NIGHTLY");
        // compile_warning(format!(">{crate_name} requires nightly"));
        ensure_nightly();
    } else {
        add_rust_valid_compile_time_flag("IS_NIGHTLY");
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

/// Rerun the `build.rs` itself file changes
pub fn rerun_if_build_rs_changes() {
    rerun_if_file_changes("build.rs");
}
/// Rerun the `build.rs` when a file changes
pub fn rerun_if_file_changes<T: std::fmt::Display>(file: T) {
    println!("cargo:rerun-if-changed={file}");
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
