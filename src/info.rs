#![allow(clippy::missing_errors_doc)]
use std::env::var as get_env;
/// Fetches the environment variable key from the current process and splits the result
pub fn get_env_list<K: AsRef<std::ffi::OsStr>>(key: K) -> AccessEnvResultStringList {
    let output = get_env(key)?;
    Ok(output
        .split(',')
        .map(std::string::ToString::to_string)
        .collect())
}

type AccessEnvResult<T> = Result<T, std::env::VarError>;
type AccessEnvResultString = AccessEnvResult<String>;
type AccessEnvResultStringList = AccessEnvResult<Vec<String>>;

/// Get the behaviour set for what happens when the program panics
pub fn get_behavior_for_panic() -> AccessEnvResultString {
    get_env("CARGO_CFG_PANIC")
}

/// Get the behaviour set for what happens when the program panics
pub fn get_behavior_for_overflow_checks() -> AccessEnvResultString {
    get_env("CARGO_CFG_OVERFLOW_CHECKS")
}

// TODO: Add the following
// CARGO={string}
// CARGO_CFG_DEBUG_ASSERTIONS={unknown}
// CARGO_CFG_FEATURE=default,std
// CARGO_CFG_FMT_DEBUG={string}
// CARGO_CFG_RELOCATION_MODEL={string}
// CARGO_CFG_TARGET_ABI={unknown}
// CARGO_CFG_TARGET_ARCH={string}
// CARGO_CFG_TARGET_ENDIAN={string}
// CARGO_CFG_TARGET_ENV={string}
// CARGO_CFG_TARGET_FAMILY={string}
// CARGO_CFG_TARGET_FEATURE={list}
// CARGO_CFG_TARGET_HAS_ATOMIC={list}
// CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE={list}
// CARGO_CFG_TARGET_HAS_ATOMIC_PRIMITIVE_ALIGNMENT={list}
// CARGO_CFG_TARGET_HAS_RELIABLE_F128={unknown}
// CARGO_CFG_TARGET_HAS_RELIABLE_F16={unknown}
// CARGO_CFG_TARGET_HAS_RELIABLE_F16_MATH={unknown}
// CARGO_CFG_TARGET_OBJECT_FORMAT={string}
// CARGO_CFG_TARGET_OS={string}
// CARGO_CFG_TARGET_POINTER_WIDTH={int}
// CARGO_CFG_TARGET_THREAD_LOCAL={unknown}
// CARGO_CFG_TARGET_VENDOR={string}
// CARGO_CFG_UB_CHECKS={unknown}
// CARGO_ENCODED_RUSTFLAGS={unknown}
// CARGO_FEATURE_DEFAULT={bool as int}
// CARGO_FEATURE_STD={bool as int}
// CARGO_HOME={string}
// CARGO_MAKEFLAGS={string}
// CARGO_MANIFEST_DIR={string}
// CARGO_MANIFEST_PATH={string}
// CARGO_PKG_AUTHORS={string}
// CARGO_PKG_DESCRIPTION={string}
// CARGO_PKG_HOMEPAGE={unknown}
// CARGO_PKG_LICENSE={string}
// CARGO_PKG_LICENSE_FILE={string}
// CARGO_PKG_NAME={string}
// CARGO_PKG_README={string}
// CARGO_PKG_REPOSITORY={string}
// CARGO_PKG_RUST_VERSION={unknown}
// CARGO_PKG_VERSION={string}
// CARGO_PKG_VERSION_MAJOR={int}
// CARGO_PKG_VERSION_MINOR={int}
// CARGO_PKG_VERSION_PATCH={int}
// CARGO_PKG_VERSION_PRE={string}
// DEBUG={bool}
// LANG={string}
// OPT_LEVEL={int}
// LANG={string}
// COLORTERM={string}
