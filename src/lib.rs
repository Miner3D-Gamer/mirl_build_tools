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
/// - Adds `{crate_name}_present` to the compile time flags so it can be used in `#[cfg({crate_name}_present)]`
///
/// TODO:
/// - Add check for the following: if this crate has a feature "X", check if the crate importing it also imports X. If it does, X must also be activated
/// - Add check for the following: If this crate has a feature "X" and a dependency also has X, then the X feature of the dependency should also be included in the X feature of this crate
pub fn setup(require_nightly: bool) {
    // Nightly check
    if require_nightly {
        ensure_nightly();
    }
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

        std::process::exit(2);
    });
    // println!("Got name! Here `{}`", crate_name);
    add_rust_compile_time_flag(&format!("{crate_name}_present"));

    // Miri check
    let has_miri_flag = has_miri_flag(&toml);

    if has_miri_flag && !is_feature_active("miri") {
        compile_error(format!(
            "Miri used inside {crate_name} without the miri flag being activated"
        ));
    }
}
/// Checks if a feature flag is currently activated
///
/// Also returns false when a flag doesn't exist
pub fn is_feature_active<T: Into<String>>(name: T) -> bool {
    let str = name.into().replace('-', "_").to_uppercase();
    let Ok(output) = std::env::var(format!("CARGO_FEATURE_{str}")) else {
        return false;
    };
    output == "1"
}

// /// Give a compile time error when miri is used without the miri flag
// #[cfg(all(miri, not(feature = "miri")))]
// pub fn check_miri_flag_if_miri() {
//     compile_error!("You are using miri without the `miri` flag")
// }

/// Give a compile time error when miri is used without the miri flag
#[allow(clippy::missing_const_for_fn)]
#[cfg(not(all(miri, not(feature = "miri"))))]
pub fn check_miri_flag_if_miri() {}

/// Print the "nightly required" screen
pub fn print_nightly_warning() {
    eprintln!(
        "{}",
        get_nightly_pretty_print().to_text(BorderVariants::determine_codec())
    );
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

// /// Checks if the project will require nightly to function
// pub fn detect_nightly() -> bool {
//     // TODO
// }

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
