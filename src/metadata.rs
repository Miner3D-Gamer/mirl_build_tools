use simd_json::base::{ValueAsArray, ValueAsObject, ValueAsScalar};

use crate::DependencyCheckSettings;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// A list of features that are missing the activation to a feature of a dependency
pub struct FeatureFlagsMissingDependencies {
    /// A list of features that are missing the activation to a feature of a dependency
    pub flags: std::collections::HashMap<String, Vec<(String, bool)>>,
    /// The features of the parent
    pub parent_features: CrateFeatures,
}
impl FeatureFlagsMissingDependencies {
    /// Add which flag is missing a link to a dependency
    pub fn add_missing_dependency_flag(
        &mut self,
        feature: std::borrow::Cow<String>,
        dependency: String,
        is_weak: bool,
    ) {
        if let Some(existing) = self.flags.get_mut(feature.as_ref()) {
            existing.push((dependency, is_weak));
        } else {
            self.flags.insert(
                feature.into_owned(),
                std::vec::Vec::from([(dependency, is_weak)]),
            );
        }
    }
}

/// Errors that might occur when trying to do a feature/dependency check
#[derive(Debug)]
pub enum DoDepCheckError {
    /// [`GetCargoInfoError`]
    GetCargo(GetCargoInfoError),
    /// [`simd_json::Error`]
    Parsing(simd_json::Error),
    /// The `packages` metadata value did not have the expected array-of-objects shape
    InvalidPackagesFormat,
    /// The requested package could not be found in the workspace metadata
    PackageNotFound(String),
    /// A package's metadata was missing an expected field, or the field had an unexpected type
    MalformedPackageField {
        /// The package whose metadata was malformed
        package: String,
        /// The field (or path to the field) that was missing/malformed
        field: &'static str,
    },
}

impl std::fmt::Display for DoDepCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetCargo(e) => write!(f, "failed to get cargo metadata: {e}"),
            Self::Parsing(e) => write!(f, "failed to parse cargo metadata: {e}"),
            Self::InvalidPackagesFormat => write!(
                f,
                "the `packages` field in cargo metadata was not an array of objects"
            ),
            Self::PackageNotFound(package) => {
                write!(
                    f,
                    "package `{package}` was not found in the workspace metadata"
                )
            }
            Self::MalformedPackageField { package, field } => write!(
                f,
                "package `{package}` had a missing or malformed `{field}` field"
            ),
        }
    }
}

impl std::error::Error for DoDepCheckError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let e: Option<&(dyn std::error::Error + 'static)> = match self {
            Self::GetCargo(e) => Some(e),
            Self::Parsing(e) => Some(e),
            Self::InvalidPackagesFormat
            | Self::PackageNotFound(_)
            | Self::MalformedPackageField { .. } => None,
        };
        e
    }
}

impl From<GetCargoInfoError> for DoDepCheckError {
    fn from(value: GetCargoInfoError) -> Self {
        Self::GetCargo(value)
    }
}
impl From<simd_json::Error> for DoDepCheckError {
    fn from(value: simd_json::Error) -> Self {
        Self::Parsing(value)
    }
}

/// Check if the package has any inconsistencies in terms of features and their relation with the used dependencies
///
/// # Errors
/// [`DoDepCheckError`]
pub fn do_dep_feature_check_for_current(
    name: &str,
    settings: &DependencyCheckSettings,
) -> Result<FeatureFlagsMissingDependencies, DoDepCheckError> {
    let mut lock = get_cargo_metadata()?;
    let value: simd_json::value::owned::Value =
        simd_json::deserialize(unsafe { lock.as_bytes_mut() })?;

    let metadata = value
        .as_object()
        .ok_or(DoDepCheckError::InvalidPackagesFormat)?
        .get("packages")
        .ok_or(DoDepCheckError::InvalidPackagesFormat)?;
    let mut metadata = CargoMetaDataPackageInfo::new_from_packages(metadata.clone())?;

    do_all_features_call_dependency_features_with_same_name(&mut metadata, name, settings)
}

/// Check if a package has its features flags correctly defined
///
/// # Errors
/// [`DoDepCheckError`]
pub fn do_all_features_call_dependency_features_with_same_name(
    meta_data_info: &mut CargoMetaDataPackageInfo,
    package_name: &str,
    settings: &DependencyCheckSettings,
) -> Result<FeatureFlagsMissingDependencies, DoDepCheckError> {
    let features = meta_data_info
        .get_features_of_package(package_name)?
        .clone();
    let dependencies = meta_data_info
        .get_dependencies_of_package(package_name)?
        .clone();

    let mut missing = FeatureFlagsMissingDependencies::default();

    for dep in dependencies.dependencies {
        if dep.kind != DependencyKind::Normal {
            continue;
        }
        if settings.blacklisted_dependencies.contains(&dep.name)
            ^ settings.blacklisted_dependencies_is_whitelist
        {
            // crate::control::compile_warning("HERE1");
            continue;
        }
        let dep_features = meta_data_info.get_features_of_package(&dep.name)?;
        let common_features = features.get_matching_other(dep_features);
        // crate::control::compile_warning(format!("{:?}", common));
        for common_feature in common_features {
            if settings.exclude_default && common_feature.name == "default" {
                continue;
            }
            if settings.exclude_flags_with_leading_underscore && common_feature.name[0..1].eq("_") {
                continue;
            }

            let add_feature = !common_feature.activating.iter().any(|x| {
                if let ActivateFeature::DependencyFeature {
                    dep: dep2,
                    feature: activating_feature,
                    weak: _,
                } = x
                {
                    return dep.name.eq(dep2) && common_feature.name.eq(activating_feature);
                }
                false
            }) && !settings
                .blacklisted_dependency_features_combination
                .iter()
                .any(|(lib, feature)| {
                    // crate::compile_warning(format!(
                    //     "Blacklisted {lib}/{feature} <= {}/{}",
                    //     dep.name, common_feature.name,
                    // ));
                    lib.eq(&dep.name) && feature.eq(&common_feature.name)
                })
                && !settings.blacklisted_features.contains(&common_feature.name)
                    ^ settings.blacklisted_features_is_whitelist;

            if add_feature {
                // crate::compile_warning(format!("#### {} => {}", common_feature.name, dep.name));
                missing.add_missing_dependency_flag(
                    std::borrow::Cow::Borrowed(&common_feature.name),
                    dep.name.clone(),
                    dep.optional,
                );
            }
        }
    }

    missing.parent_features = features;

    Ok(missing)
}

#[derive(Debug)]
/// Errors that might occur when trying to get the project metadata
pub enum GetCargoInfoError {
    /// Command error
    IOError(std::io::Error),
    /// Command output is not utf8
    NotUtf8(std::string::FromUtf8Error),
    /// Command error with error message
    StErr(String, i32),
}
impl std::fmt::Display for GetCargoInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "IO error: {e}"),
            Self::NotUtf8(e) => write!(f, "output was not valid UTF-8: {e}"),
            Self::StErr(stderr, code) => {
                write!(f, "command exited with status {code}: {stderr}")
            }
        }
    }
}

impl std::error::Error for GetCargoInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IOError(e) => Some(e),
            Self::NotUtf8(e) => Some(e),
            Self::StErr(_, _) => None,
        }
    }
}

impl From<std::io::Error> for GetCargoInfoError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<std::string::FromUtf8Error> for GetCargoInfoError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::NotUtf8(value)
    }
}
impl From<(String, i32)> for GetCargoInfoError {
    fn from(value: (String, i32)) -> Self {
        Self::StErr(value.0, value.1)
    }
}
/// Get the metadata info about the current project
///
/// # Errors
/// [`GetCargoInfoError`]
pub fn get_cargo_metadata() -> Result<String, GetCargoInfoError> {
    let command = "cargo metadata --format-version 1 --all-features";
    let mut command = _new_command(command);

    let info = command.output()?;
    if info.status.code().unwrap_or(i32::MAX) != 0 {
        Err((
            String::from_utf8(info.stderr)?,
            info.status.code().unwrap_or(i32::MAX),
        ))?;
    }
    let text = String::from_utf8(info.stdout)?;
    Ok(text)
}
#[must_use]
/// Create a new command from the given string
pub fn _new_command(command: &str) -> std::process::Command {
    let list: Vec<&str> = command.split(' ').collect();

    let mut command = std::process::Command::new(list[0]);
    command.args(&list[1..]);
    command
}
#[derive(Debug, Clone, PartialEq)]
/// Package info about the packages
pub struct CargoMetaDataPackageInfo {
    packages: Vec<simd_json::value::owned::Object>,
    dependencies: std::collections::HashMap<String, CrateDependencies>,
    features: std::collections::HashMap<String, CrateFeatures>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Features can activate other features, dependencies, or features of dependencies
pub enum ActivateFeature {
    /// Activates a dependency
    Dependency(String),
    /// Activates a feature
    Feature(String),
    /// Activates a feature of a dependency
    DependencyFeature {
        /// The dependency that may or may not be imported
        dep: String,
        /// The feature of the dependency
        feature: String,
        /// Only activates a feature is the dependency is optional
        weak: bool,
    },
}

impl std::fmt::Display for ActivateFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dependency(dep) => write!(f, "dep:{dep}"),
            Self::Feature(feature) => f.write_str(feature),
            Self::DependencyFeature { dep, feature, weak } => {
                write!(f, "{dep}{}/{feature}", if *weak { "?" } else { "" })
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A feature defined as
/// {name} = [{activations}]
pub struct CrateFeature {
    /// Name of the feature
    pub name: String,
    /// The things it activates
    pub activating: Vec<ActivateFeature>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The list of [`CreateFeature`]
pub struct CrateFeatures {
    /// The list of [`CreateFeature`]
    pub features: Vec<CrateFeature>,
}

impl CrateFeatures {
    #[must_use]
    /// Get the features that both this and the other have in common
    pub fn get_matching_other(&self, other: &Self) -> Vec<&CrateFeature> {
        self.features
            .iter()
            .filter(|x| other.features.iter().any(|y| x.name == y.name))
            .collect()
    }

    #[must_use]
    /// Get a flag by its name
    pub fn get_flag(&self, flag: &str) -> Option<&CrateFeature> {
        self.features.iter().find(|x| x.name.eq(flag))
    }
    #[must_use]
    /// Get all feature names
    pub fn get_feature_names(&self) -> Vec<&String> {
        self.features.iter().map(|x| &x.name).collect()
    }
}
/// When the dependency is used
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyKind {
    /// Normally
    Normal,
    /// During build time
    Build,
    /// During Dev time
    Dev,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A dependency of a crate
pub struct CrateDependency {
    /// Name of the dependency
    pub name: String,
    /// Required version(s)
    pub version: String,
    /// If the dependency is optional
    pub optional: bool,
    /// When the dep is needed
    pub kind: DependencyKind,
    /// Additionally active features
    pub active_features: Vec<String>,
    /// If the default features are used
    pub use_default_features: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The list of dependencies
pub struct CrateDependencies {
    /// The list of dependencies
    pub dependencies: Vec<CrateDependency>,
}
impl CrateDependencies {
    #[must_use]
    /// Get all dependency names
    pub fn get_all_dependency_names(&self) -> Vec<&String> {
        self.dependencies.iter().map(|x| &x.name).collect()
    }
    #[must_use]
    /// Get all dependency names that are optional
    pub fn get_all_optional_dependency_names(&self) -> Vec<&String> {
        self.dependencies
            .iter()
            .filter(|x| x.optional)
            .map(|x| &x.name)
            .collect()
    }
    #[must_use]
    /// Get all dependency names that aren't optional
    pub fn get_all_non_optional_dependency_names(&self) -> Vec<&String> {
        self.dependencies
            .iter()
            .filter(|x| !x.optional)
            .map(|x| &x.name)
            .collect()
    }
    #[must_use]
    /// If the dependencies contain the given package
    pub fn contains_dep(&self, package: &str) -> bool {
        self.dependencies.iter().any(|x| x.name == package)
    }
}

impl CargoMetaDataPackageInfo {
    /// Create a new [`CargoMetaDataPackageInfo`] from the output of a [`simd_json::value::owned::Value`]
    ///
    /// # Errors
    /// [`DoDepCheckError::InvalidPackagesFormat`] if `packages` isn't an array of objects
    pub fn new_from_packages(
        packages: simd_json::value::owned::Value,
    ) -> Result<Self, DoDepCheckError> {
        let simd_json::value::owned::Value::Array(list) = packages else {
            return Err(DoDepCheckError::InvalidPackagesFormat);
        };

        Ok(Self {
            packages: list
                .into_iter()
                .map(|x| {
                    let simd_json::value::owned::Value::Object(obj) = x else {
                        return None;
                    };
                    Some(*obj)
                })
                .collect::<Option<Vec<simd_json::value::owned::Object>>>()
                .ok_or(DoDepCheckError::InvalidPackagesFormat)?,
            dependencies: std::collections::HashMap::default(),
            features: std::collections::HashMap::default(),
        })
    }

    /// Get the features a package holds
    ///
    /// # Errors
    /// [`DoDepCheckError`]
    pub fn get_features_of_package(
        &mut self,
        package: &str,
    ) -> Result<&CrateFeatures, DoDepCheckError> {
        if !self.features.contains_key(package) {
            self.process_package(package)?;
        }

        self.features
            .get(package)
            .ok_or_else(|| DoDepCheckError::PackageNotFound(package.to_string()))
    }
    /// Get the dependencies a package holds
    ///
    /// # Errors
    /// [`DoDepCheckError`]
    pub fn get_dependencies_of_package(
        &mut self,
        package: &str,
    ) -> Result<&CrateDependencies, DoDepCheckError> {
        if !self.dependencies.contains_key(package) {
            self.process_package(package)?;
        }

        self.dependencies
            .get(package)
            .ok_or_else(|| DoDepCheckError::PackageNotFound(package.to_string()))
    }
    /// Process a given crate, extracting their dependencies and features
    ///
    /// # Errors
    /// [`DoDepCheckError`]
    pub fn process_package(&mut self, package: &str) -> Result<(), DoDepCheckError> {
        let idx = self
            .get_idx_of_package(package)
            .ok_or_else(|| DoDepCheckError::PackageNotFound(package.to_string()))?;

        let p = self.packages.remove(idx);
        {
            let dependencies = p
                .get("dependencies")
                .and_then(ValueAsArray::as_array)
                .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                    package: package.to_string(),
                    field: "dependencies",
                })?;

            let mut dep_list = CrateDependencies::default();

            for dep in dependencies {
                let dep =
                    dep.as_object()
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].<object>",
                        })?;
                let dependency = CrateDependency {
                    name: dep
                        .get("name")
                        .and_then(ValueAsScalar::as_str)
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].name",
                        })?
                        .to_string(),
                    version: dep
                        .get("req")
                        .and_then(ValueAsScalar::as_str)
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].req",
                        })?
                        .to_string(),
                    optional: dep
                        .get("optional")
                        .and_then(ValueAsScalar::as_bool)
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].optional",
                        })?,
                    active_features: dep
                        .get("features")
                        .and_then(ValueAsArray::as_array)
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].features",
                        })?
                        .iter()
                        .map(|x| x.as_str().map(std::string::ToString::to_string))
                        .collect::<Option<Vec<String>>>()
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].features[]",
                        })?,
                    use_default_features: dep
                        .get("uses_default_features")
                        .and_then(ValueAsScalar::as_bool)
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "dependencies[].uses_default_features",
                        })?,
                    kind: {
                        let dep = dep
                            .get("kind")
                            .and_then(ValueAsScalar::as_str)
                            .unwrap_or("");
                        match dep {
                            "build" => DependencyKind::Build,
                            "dev" => DependencyKind::Dev,
                            "" => DependencyKind::Normal,
                            _ => {
                                return Err(DoDepCheckError::MalformedPackageField {
                                    package: format!("Package: {package}, Found value: '{dep}'"),
                                    field: "dependencies[].kind",
                                });
                            }
                        }
                    },
                };
                dep_list.dependencies.push(dependency);
            }

            {
                let features = p
                    .get("features")
                    .and_then(ValueAsObject::as_object)
                    .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                        package: package.to_string(),
                        field: "features",
                    })?;

                let mut feature_list = Vec::default();

                for (name, activations) in features {
                    let activations = activations
                        .as_array()
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "features[].<value>",
                        })?
                        .iter()
                        .map(|x| x.as_str().map(std::string::ToString::to_string))
                        .collect::<Option<Vec<String>>>()
                        .ok_or_else(|| DoDepCheckError::MalformedPackageField {
                            package: package.to_string(),
                            field: "features[].<value>[]",
                        })?;

                    let mut feature = CrateFeature {
                        name: name.clone(),
                        activating: Vec::default(),
                    };

                    for feature_activation in activations {
                        let t = if feature_activation.starts_with("dep:") {
                            let mut feature_activation = feature_activation;
                            for _ in 0.."dep:".len() {
                                feature_activation.remove(0);
                            }
                            ActivateFeature::Dependency(feature_activation)
                        } else if let Some((name, feature)) = feature_activation.split_once('/') {
                            let weak = name.ends_with('?');
                            let name = if weak { &name[..name.len() - 1] } else { name };
                            ActivateFeature::DependencyFeature {
                                dep: name.to_string(),
                                feature: feature.to_string(),
                                weak,
                            }
                        } else if dep_list.contains_dep(&feature_activation) {
                            ActivateFeature::Dependency(feature_activation)
                        } else {
                            // crate::compile_warning(format!("#### {feature_activation}"));
                            ActivateFeature::Feature(feature_activation)
                        };
                        feature.activating.push(t);
                    }
                    feature_list.push(feature);
                }
                // Fix of feature_list: Dependencies are preferred over features which should be the opposite except if the feature has the same name as the dependency

                let mut feature_list = CrateFeatures {
                    features: feature_list,
                };
                let feature_names: Vec<String> = feature_list
                    .get_feature_names()
                    .into_iter()
                    .cloned()
                    .collect();

                for feature in &mut feature_list.features {
                    for activating in &mut feature.activating {
                        if let ActivateFeature::Dependency(name) = activating
                            && feature_names.contains(name)
                            && !feature.name.eq(name)
                        {
                            let name = std::mem::take(name);
                            *activating = ActivateFeature::Feature(name);
                        }
                    }
                }

                self.features.insert(package.to_string(), feature_list);
            }

            self.dependencies.insert(package.to_string(), dep_list);
        }

        Ok(())
    }
    /// Get the idx of the package with the given name
    ///
    /// Returns `None` if no package with that name exists — this is the only meaning
    /// `None` can carry here, so it stays an `Option` rather than a `DoDepCheckError`.
    #[must_use]
    pub fn get_idx_of_package(&self, package: &str) -> Option<usize> {
        let idx = self.packages.iter().position(|value| {
            let Some(v) = value.get("name").and_then(|x| x.as_str()) else {
                return false;
            };
            v == package
        })?;
        Some(idx)
    }
}
