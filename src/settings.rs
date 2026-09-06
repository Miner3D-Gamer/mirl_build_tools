#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Settings for the setup
pub struct SetupSettings {
    /// Nightly behavior
    pub nightly: NightlySettings,
    /// If `CRATE_PRESENT_{crate_name.to_uppercase()}` should be added to the env to be used in `#[cfg(CRATE_PRESENT_{crate_name.to_uppercase()})]`
    pub set_crate_present_cfg: bool,
    /// If for flag activations should be checked for when a supported flag is defined
    pub flag_condition_check: FlagConditionCheck,
    /// Dependency check the current crate for matching features
    pub dependency_check: DependencyCheck,
}
impl Default for SetupSettings {
    fn default() -> Self {
        Self {
            nightly: NightlySettings::default(),
            set_crate_present_cfg: true,
            flag_condition_check: FlagConditionCheck::default(),
            dependency_check: DependencyCheck::default(),
        }
    }
}
/// How the program should handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum NightlySettings {
    /// You are using nightly and wish for this to be reflected
    #[default]
    OverwriteWithNightly,
    /// You are using stable and wish for this to be reflected
    OverwriteWithStable,
    /// You may or may not be using nighty, automated detection will tell
    DetectAutomatically,
}
/// If for flag activations should be checked for when a supported flag is defined
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlagConditionCheck {
    /// Check for miri flag to be active
    pub miri: bool,
    /// Check for test flag to be active
    pub test: bool,
}
impl Default for FlagConditionCheck {
    fn default() -> Self {
        Self {
            miri: true,
            test: true,
        }
    }
}
/// How the dependency check should be handled
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyCheck {
    /// Do not check the dependencies
    DoNotCheck,
    /// Check the dependencies with the following settings
    Check(DependencyCheckSettings),
}
impl Default for DependencyCheck {
    fn default() -> Self {
        Self::Check(DependencyCheckSettings::default())
    }
}
/// The settings for the dependency check
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyCheckSettings {
    /// Exclude the "default" flag
    pub exclude_default: bool,
    /// Exclude all features that start with an underscore
    pub exclude_flags_with_leading_underscore: bool,
    /// What dependency types are blacklisted
    pub blacklisted_dependency_kind: SelectedDependencyType,
    /// [`blacklisted_dependency_kind`](Self::blacklisted_dependency_kind) should be treated as whitelist instead
    pub blacklisted_dependency_kind_is_whitelist: bool,
    /// What dependencies should be skipped
    pub blacklisted_dependencies: Vec<String>,
    /// [`blacklisted_dependency_kind`](Self::blacklisted_dependencies) should be treated as whitelist instead
    pub blacklisted_dependencies_is_whitelist: bool,
    /// What features should be skipped
    pub blacklisted_features: Vec<String>,
    /// [`blacklisted_dependency_kind`](Self::blacklisted_features) should be treated as whitelist instead
    pub blacklisted_features_is_whitelist: bool,
    /// What combination of dependency and feature should be skipped, formatted as (Dep, Flag)
    pub blacklisted_dependency_features_combination: Vec<(String, String)>,
}
impl Default for DependencyCheckSettings {
    fn default() -> Self {
        Self {
            exclude_default: true,
            exclude_flags_with_leading_underscore: true,
            blacklisted_dependency_kind: SelectedDependencyType::default(),
            blacklisted_dependency_kind_is_whitelist: true,
            blacklisted_dependencies: Vec::new(),
            blacklisted_dependencies_is_whitelist: false,
            blacklisted_features: Vec::new(),
            blacklisted_features_is_whitelist: false,
            blacklisted_dependency_features_combination: Vec::new(),
        }
    }
}
/// Selected dependency types, a dependency can either be normal, build, or dev
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectedDependencyType {
    /// Normal crates
    pub normal: bool,
    /// Crates that are only used using compile time
    pub build: bool,
    /// Crates that are only used in dev
    pub dev: bool,
}

impl Default for SelectedDependencyType {
    fn default() -> Self {
        Self {
            normal: true,
            build: false,
            dev: false,
        }
    }
}
impl AsRef<Self> for SetupSettings {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl SetupSettings {
    /// Overwrite the [`NightlySettings`] wholesale.
    pub const fn set_nightly(&mut self, nightly: NightlySettings) -> &mut Self {
        self.nightly = nightly;
        self
    }

    /// Shortcut: force nightly behavior to be reported as active.
    pub const fn set_nightly_overwrite_with_nightly(&mut self) -> &mut Self {
        self.nightly = NightlySettings::OverwriteWithNightly;
        self
    }

    /// Shortcut: force nightly behavior to be reported as inactive (stable).
    pub const fn set_nightly_overwrite_with_stable(&mut self) -> &mut Self {
        self.nightly = NightlySettings::OverwriteWithStable;
        self
    }

    /// Shortcut: let nightly/stable be detected automatically.
    pub const fn set_nightly_detect_automatically(&mut self) -> &mut Self {
        self.nightly = NightlySettings::DetectAutomatically;
        self
    }

    /// Get mutable access to the nested [`NightlySettings`] for further chaining.
    pub const fn nightly_mut(&mut self) -> &mut NightlySettings {
        &mut self.nightly
    }

    /// Set whether `CRATE_PRESENT_{CRATE}` cfgs should be emitted.
    pub const fn set_crate_present_cfg(&mut self, enabled: bool) -> &mut Self {
        self.set_crate_present_cfg = enabled;
        self
    }

    /// Convenience: enable `CRATE_PRESENT_{CRATE}` cfg emission.
    pub const fn enable_crate_present_cfg(&mut self) -> &mut Self {
        self.set_crate_present_cfg(true)
    }

    /// Convenience: disable `CRATE_PRESENT_{CRATE}` cfg emission.
    pub const fn disable_crate_present_cfg(&mut self) -> &mut Self {
        self.set_crate_present_cfg(false)
    }

    /// Overwrite the [`FlagConditionCheck`] wholesale.
    pub const fn set_flag_condition_check(&mut self, check: FlagConditionCheck) -> &mut Self {
        self.flag_condition_check = check;
        self
    }

    /// Get mutable access to the nested [`FlagConditionCheck`] for further chaining.
    pub const fn flag_condition_check_mut(&mut self) -> &mut FlagConditionCheck {
        &mut self.flag_condition_check
    }

    /// Overwrite the [`DependencyCheck`] wholesale.
    pub fn set_dependency_check(&mut self, check: DependencyCheck) -> &mut Self {
        self.dependency_check = check;
        self
    }

    /// Convenience: disable dependency checking entirely.
    pub fn disable_dependency_check(&mut self) -> &mut Self {
        self.dependency_check = DependencyCheck::DoNotCheck;
        self
    }

    /// Convenience: enable dependency checking with default settings, unless
    /// it is already enabled (in which case existing settings are kept).
    pub fn enable_dependency_check(&mut self) -> &mut Self {
        if !matches!(self.dependency_check, DependencyCheck::Check(_)) {
            self.dependency_check = DependencyCheck::Check(DependencyCheckSettings::default());
        }
        self
    }

    /// Get mutable access to the nested [`DependencyCheck`] for further chaining.
    pub const fn dependency_check_mut(&mut self) -> &mut DependencyCheck {
        &mut self.dependency_check
    }
    /// Get mutable access to the nested [`DependencyCheck`] for further chaining.
    pub fn with_enabled_dependency_check_settings(
        &mut self,
        closure: impl Fn(&mut DependencyCheckSettings),
    ) -> &mut Self {
        if !matches!(self.dependency_check, DependencyCheck::Check(_)) {
            self.dependency_check = DependencyCheck::Check(DependencyCheckSettings::default());
        }

        if let DependencyCheck::Check(var) = &mut self.dependency_check {
            closure(var);
            return self;
        }
        #[allow(clippy::unnecessary_literal_unwrap)]
        unsafe {
            None.unwrap_unchecked()
        }
    }

    /// Reset the entire struct back to its default values.
    pub fn reset(&mut self) -> &mut Self {
        *self = Self::default();
        self
    }
}

impl NightlySettings {
    /// Set this to [`NightlySettings::OverwriteWithNightly`].
    pub const fn set_overwrite_with_nightly(&mut self) -> &mut Self {
        *self = Self::OverwriteWithNightly;
        self
    }

    /// Set this to [`NightlySettings::OverwriteWithStable`].
    pub const fn set_overwrite_with_stable(&mut self) -> &mut Self {
        *self = Self::OverwriteWithStable;
        self
    }

    /// Set this to [`NightlySettings::DetectAutomatically`].
    pub const fn set_detect_automatically(&mut self) -> &mut Self {
        *self = Self::DetectAutomatically;
        self
    }

    /// Directly assign a new value.
    pub const fn set(&mut self, value: Self) -> &mut Self {
        *self = value;
        self
    }
}

impl FlagConditionCheck {
    /// Set whether the `miri` flag should be checked.
    pub const fn set_miri(&mut self, enabled: bool) -> &mut Self {
        self.miri = enabled;
        self
    }

    /// Set whether the `test` flag should be checked.
    pub const fn set_test(&mut self, enabled: bool) -> &mut Self {
        self.test = enabled;
        self
    }

    /// Enable checking for the `miri` flag.
    pub const fn enable_miri(&mut self) -> &mut Self {
        self.set_miri(true)
    }

    /// Disable checking for the `miri` flag.
    pub const fn disable_miri(&mut self) -> &mut Self {
        self.set_miri(false)
    }

    /// Enable checking for the `test` flag.
    pub const fn enable_test(&mut self) -> &mut Self {
        self.set_test(true)
    }

    /// Disable checking for the `test` flag.
    pub const fn disable_test(&mut self) -> &mut Self {
        self.set_test(false)
    }

    /// Set both `miri` and `test` checks to the same value at once.
    pub const fn set_all(&mut self, enabled: bool) -> &mut Self {
        self.miri = enabled;
        self.test = enabled;
        self
    }

    /// Enable all flag condition checks.
    pub const fn enable_all(&mut self) -> &mut Self {
        self.set_all(true)
    }

    /// Disable all flag condition checks.
    pub const fn disable_all(&mut self) -> &mut Self {
        self.set_all(false)
    }
}

impl DependencyCheck {
    /// Set this to [`DependencyCheck::DoNotCheck`].
    pub fn set_do_not_check(&mut self) -> &mut Self {
        *self = Self::DoNotCheck;
        self
    }

    /// Set this to [`DependencyCheck::Check`] with the provided settings.
    pub fn set_check(&mut self, settings: DependencyCheckSettings) -> &mut Self {
        *self = Self::Check(settings);
        self
    }

    /// Set this to [`DependencyCheck::Check`] with default settings.
    pub fn set_check_default(&mut self) -> &mut Self {
        self.set_check(DependencyCheckSettings::default())
    }

    /// Directly assign a new value.
    pub fn set(&mut self, value: Self) -> &mut Self {
        *self = value;
        self
    }

    /// Get mutable access to the inner [`DependencyCheckSettings`], if this is
    /// currently the `Check` variant. Returns `None` for `DoNotCheck`.
    pub const fn settings_mut(&mut self) -> Option<&mut DependencyCheckSettings> {
        match self {
            Self::Check(settings) => Some(settings),
            Self::DoNotCheck => None,
        }
    }

    /// Ensure this is the `Check` variant (switching to default settings if
    /// it wasn't already), then return mutable access to its settings.
    pub fn ensure_check_mut(&mut self) -> &mut DependencyCheckSettings {
        if !matches!(self, Self::Check(_)) {
            *self = Self::Check(DependencyCheckSettings::default());
        }
        match self {
            Self::Check(settings) => settings,
            Self::DoNotCheck => unreachable!(),
        }
    }
}

impl DependencyCheckSettings {
    /// Set whether the "default" feature should be excluded.
    pub const fn set_exclude_default(&mut self, exclude: bool) -> &mut Self {
        self.exclude_default = exclude;
        self
    }

    /// Set whether features starting with `_` should be excluded.
    pub const fn set_exclude_flags_with_leading_underscore(&mut self, exclude: bool) -> &mut Self {
        self.exclude_flags_with_leading_underscore = exclude;
        self
    }

    /// Overwrite the blacklisted dependency kind wholesale.
    pub const fn set_blacklisted_dependency_kind(
        &mut self,
        kind: SelectedDependencyType,
    ) -> &mut Self {
        self.blacklisted_dependency_kind = kind;
        self
    }

    /// Get mutable access to the blacklisted dependency kind for chaining.
    pub const fn blacklisted_dependency_kind_mut(&mut self) -> &mut SelectedDependencyType {
        &mut self.blacklisted_dependency_kind
    }

    /// Set whether `blacklisted_dependency_kind` should be treated as a whitelist.
    pub const fn set_blacklisted_dependency_kind_is_whitelist(
        &mut self,
        is_whitelist: bool,
    ) -> &mut Self {
        self.blacklisted_dependency_kind_is_whitelist = is_whitelist;
        self
    }

    /// Overwrite the list of blacklisted dependencies wholesale.
    pub fn set_blacklisted_dependencies(&mut self, dependencies: Vec<String>) -> &mut Self {
        self.blacklisted_dependencies = dependencies;
        self
    }

    /// Add a single dependency to the blacklist (no-op if already present).
    pub fn add_blacklisted_dependency(&mut self, dependency: impl Into<String>) -> &mut Self {
        let dependency = dependency.into();
        if !self.blacklisted_dependencies.contains(&dependency) {
            self.blacklisted_dependencies.push(dependency);
        }
        self
    }

    /// Add multiple dependencies to the blacklist at once.
    pub fn add_blacklisted_dependencies<I, S>(&mut self, dependencies: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for dependency in dependencies {
            self.add_blacklisted_dependency(dependency);
        }
        self
    }

    /// Remove a single dependency from the blacklist, if present.
    pub fn remove_blacklisted_dependency(&mut self, dependency: &str) -> &mut Self {
        self.blacklisted_dependencies.retain(|d| d != dependency);
        self
    }

    /// Clear all blacklisted dependencies.
    pub fn clear_blacklisted_dependencies(&mut self) -> &mut Self {
        self.blacklisted_dependencies.clear();
        self
    }

    /// Set whether `blacklisted_dependencies` should be treated as a whitelist.
    pub const fn set_blacklisted_dependencies_is_whitelist(
        &mut self,
        is_whitelist: bool,
    ) -> &mut Self {
        self.blacklisted_dependencies_is_whitelist = is_whitelist;
        self
    }

    /// Overwrite the list of blacklisted features wholesale.
    pub fn set_blacklisted_features(&mut self, features: Vec<String>) -> &mut Self {
        self.blacklisted_features = features;
        self
    }

    /// Add a single feature to the blacklist (no-op if already present).
    pub fn add_blacklisted_feature(&mut self, feature: impl Into<String>) -> &mut Self {
        let feature = feature.into();
        if !self.blacklisted_features.contains(&feature) {
            self.blacklisted_features.push(feature);
        }
        self
    }

    /// Add multiple features to the blacklist at once.
    pub fn add_blacklisted_features<I, S>(&mut self, features: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for feature in features {
            self.add_blacklisted_feature(feature);
        }
        self
    }

    /// Remove a single feature from the blacklist, if present.
    pub fn remove_blacklisted_feature(&mut self, feature: &str) -> &mut Self {
        self.blacklisted_features.retain(|f| f != feature);
        self
    }

    /// Clear all blacklisted features.
    pub fn clear_blacklisted_features(&mut self) -> &mut Self {
        self.blacklisted_features.clear();
        self
    }

    /// Set whether `blacklisted_features` should be treated as a whitelist.
    pub const fn set_blacklisted_features_is_whitelist(&mut self, is_whitelist: bool) -> &mut Self {
        self.blacklisted_features_is_whitelist = is_whitelist;
        self
    }

    /// Overwrite the list of blacklisted (dependency, feature) combinations wholesale.
    pub fn set_blacklisted_dependency_features_combination(
        &mut self,
        combinations: Vec<(String, String)>,
    ) -> &mut Self {
        self.blacklisted_dependency_features_combination = combinations;
        self
    }

    /// Add a single (dependency, feature) combination to the blacklist
    /// (no-op if already present).
    pub fn add_blacklisted_dependency_feature_combination(
        &mut self,
        dependency: impl Into<String>,
        feature: impl Into<String>,
    ) -> &mut Self {
        let pair = (dependency.into(), feature.into());
        if !self
            .blacklisted_dependency_features_combination
            .contains(&pair)
        {
            self.blacklisted_dependency_features_combination.push(pair);
        }
        self
    }

    /// Remove a single (dependency, feature) combination from the blacklist, if present.
    pub fn remove_blacklisted_dependency_feature_combination(
        &mut self,
        dependency: &str,
        feature: &str,
    ) -> &mut Self {
        self.blacklisted_dependency_features_combination
            .retain(|(d, f)| !(d == dependency && f == feature));
        self
    }

    /// Clear all blacklisted (dependency, feature) combinations.
    pub fn clear_blacklisted_dependency_features_combination(&mut self) -> &mut Self {
        self.blacklisted_dependency_features_combination.clear();
        self
    }

    /// Reset this struct back to its default values.
    pub fn reset(&mut self) -> &mut Self {
        *self = Self::default();
        self
    }
}

impl SelectedDependencyType {
    /// Set whether normal dependencies are selected.
    pub const fn set_normal(&mut self, enabled: bool) -> &mut Self {
        self.normal = enabled;
        self
    }

    /// Set whether build dependencies are selected.
    pub const fn set_build(&mut self, enabled: bool) -> &mut Self {
        self.build = enabled;
        self
    }

    /// Set whether dev dependencies are selected.
    pub const fn set_dev(&mut self, enabled: bool) -> &mut Self {
        self.dev = enabled;
        self
    }

    /// Set normal, build, and dev all to the same value at once.
    pub const fn set_all(&mut self, enabled: bool) -> &mut Self {
        self.normal = enabled;
        self.build = enabled;
        self.dev = enabled;
        self
    }

    /// Select every dependency kind (normal, build, dev).
    pub const fn enable_all(&mut self) -> &mut Self {
        self.set_all(true)
    }

    /// Deselect every dependency kind (normal, build, dev).
    pub const fn disable_all(&mut self) -> &mut Self {
        self.set_all(false)
    }

    /// Toggle the normal flag.
    pub const fn toggle_normal(&mut self) -> &mut Self {
        self.normal = !self.normal;
        self
    }

    /// Toggle the build flag.
    pub const fn toggle_build(&mut self) -> &mut Self {
        self.build = !self.build;
        self
    }

    /// Toggle the dev flag.
    pub const fn toggle_dev(&mut self) -> &mut Self {
        self.dev = !self.dev;
        self
    }
}
