
/// Add a flag that can be checked at compile time
///
/// # Usage:
///
/// Build:
///
/// ```no_run
/// add_rust_compile_time_flag("flag");
/// ```
///
/// Compile:
/// ```no_run
/// #[cfg(flag)]
/// ```
pub fn add_rust_compile_time_flag(flag: &str) {
    println!("cargo:rustc-cfg={flag}");
}
/// Give out a compile time warning
pub fn compile_warning<T: std::fmt::Display>(warning: T) {
    println!("cargo:warning={warning}");
}

/// Give out a compile time error
pub fn compile_error<T: std::fmt::Display>(error: T) {
    println!("cargo:error={error}");
}
/// Exit the process
/// 
/// Equivalent to `std::process::exit(code)`
pub fn exit<T: Into<i32>>(code: T) {
    std::process::exit(code.into())
}
/// Exit the process
/// 
/// Equivalent to `std::process::exit(code)`
pub fn quit<T: Into<i32>>(code: T) {
    exit(code);
}