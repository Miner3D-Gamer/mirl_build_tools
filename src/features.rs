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
