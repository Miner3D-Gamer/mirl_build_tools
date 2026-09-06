use crate::{
    get_lib_rs_path, get_main_rs_path,
    pretty_print::{BorderVariants, get_nightly_pretty_print},
};

/// Checks if the project will require nightly to function
///
/// # Errors
/// When accessing the source code file is not possible
/// 
/// TODO: This function seems to leave false negatives!!!
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
}
