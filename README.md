# Mirl Build Tools (0.0.0-alpha)

### Mibits - Helper functionality for `build.rs`

[![Version](https://img.shields.io/crates/v/mirl_build_tools.svg)](https://crates.io/crates/mirl_build_tools)
[![Documentation](https://docs.rs/mirl_build_tools/badge.svg)](https://docs.rs/mirl_build_tools)
[![dependency status](https://deps.rs/repo/github/Miner3D-Gamer/mirl_build_tools/status.svg)](https://deps.rs/repo/github/Miner3D-Gamer/mirl_build_tools)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.99–nightly-blue)](https://github.com/rust-lang/rust/releases/tag/1.99.0)
[![Changelog](https://img.shields.io/badge/CHANGELOG.md--555.svg)](https://github.com/Miner3D-Gamer/mirl_build_tools/blob/main/CHANGELOG.md)

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required
- `c_compatible`
- `all`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

**Enum**

- `all_enum_extensions`
- `strum`
- `enum_ext`

### Custom:

- `margin` > `border` - Insert a margin between the text and the border when formatting
- `border` - If the should be a border surrounding the text when formatting
- `force_unicode` - Force the border to use unicode compatible formatting
- `force_ascii` - Force the border to use an ascii compatible formatting
- `force_code_page_437` - Force the border to use a Code Page 437 compatible formatting

</details>

### Entry points

The `setup` function does 4 things (All configurable):

1. Ensures that nightly is used, warning the user otherwise.
2. Checks if the crate has a `miri` flag. If miri is used without the flag being set, gives user an error.
3. Adds presents to compilation environment under `{crate_name}_present`
4. If this crate has a flag and a dependency has a flag with the same name, the flag in the current lib should call the flag in the dependency. If not, a warning appears.

### Purpose

Detect if nightly is in use and warn the User if not using a single function call

### Disclaimer

This lib is meant to be used purely for the `build.rs` setup though it does contain a custom pretty print formatter one could use elsewhere.

### Origin

The thought of copy pasting detection/warning functions into the `build.rs` of every mirl crate disgusted me so this lib exists instead.

### TODO

Overhaul the dependency detection.

Instead of using raw data for the flag detection, collect all data and build a "generated" folder with different things (traits, dependencies). Also relay dependency flag existence and other info to the `Mirl Macro Setup`.
