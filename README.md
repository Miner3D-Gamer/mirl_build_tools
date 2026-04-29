# Mirl Build Tools (0.0.0-alpha)

### Mibits - Helper functionality for `build.rs`

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required
- `c_compatible`

**Codec**

- `serde`
- `bitcode`
- `wincode` (bitcode recommended)

**Enum**

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

The `ensure_nightly` function ensures that nightly is used, warns the user otherwise.

### Purpose

Detect if nightly is in use and warn the User if not using a single function call

### Disclaimer

This lib is meant to be used purely for the nightly detection/warning though it does contain a custom pretty print formatter one could use elsewhere.

### Origin

The thought of the detection/warning functions into every mirl crate disgusted me so instead this lib exists.
