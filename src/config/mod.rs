use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub behavior: BehaviorConfig,
    #[serde(default)]
    pub categories: BTreeMap<String, CategoryRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehaviorConfig {
    #[serde(default = "default_unknown_files")]
    pub unknown_files: UnknownFilesBehavior,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub follow_symlinks: bool,
    #[serde(default = "default_conflict_strategy")]
    pub conflict: ConflictStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum UnknownFilesBehavior {
    Other,
    Skip,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictStrategy {
    Rename,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CategoryRule {
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub filename_starts_with: Vec<String>,
    #[serde(default)]
    pub filename_contains: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigValidationError {
    pub message: String,
}

impl ConfigValidationError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            behavior: BehaviorConfig::default(),
            categories: BTreeMap::new(),
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            unknown_files: default_unknown_files(),
            include_hidden: false,
            recursive: false,
            follow_symlinks: false,
            conflict: default_conflict_strategy(),
        }
    }
}

fn default_unknown_files() -> UnknownFilesBehavior {
    UnknownFilesBehavior::Other
}

fn default_conflict_strategy() -> ConflictStrategy {
    ConflictStrategy::Rename
}

pub fn load_config_from_path(path: &Path) -> io::Result<Config> {
    let yaml = fs::read_to_string(path)?;

    serde_yaml::from_str(&yaml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse config: {error}"),
        )
    })
}

pub fn validate_config(config: &Config) -> Result<(), ConfigValidationError> {
    if config.version != 1 {
        return Err(ConfigValidationError::new(format!(
            "unsupported config version: {}",
            config.version
        )));
    }

    for (category_name, rule) in &config.categories {
        validate_category_name(category_name)?;

        if rule.extensions.is_empty()
            && rule.filename_starts_with.is_empty()
            && rule.filename_contains.is_empty()
        {
            return Err(ConfigValidationError::new(format!(
                "category '{category_name}' has no match conditions"
            )));
        }

        for extension in &rule.extensions {
            validate_extension(category_name, extension)?;
        }
    }

    Ok(())
}

fn validate_category_name(category_name: &str) -> Result<(), ConfigValidationError> {
    if category_name.trim().is_empty() {
        return Err(ConfigValidationError::new("category name cannot be empty"));
    }

    if category_name.contains('/') || category_name.contains('\\') {
        return Err(ConfigValidationError::new(format!(
            "category '{category_name}' must be a single directory name"
        )));
    }

    if category_name.contains("..") {
        return Err(ConfigValidationError::new(format!(
            "category '{category_name}' cannot contain path traversal"
        )));
    }

    if category_name.starts_with('~') || category_name.starts_with(':') {
        return Err(ConfigValidationError::new(format!(
            "category '{category_name}' cannot be absolute"
        )));
    }

    if looks_like_windows_absolute_path(category_name) {
        return Err(ConfigValidationError::new(format!(
            "category '{category_name}' cannot be absolute"
        )));
    }

    Ok(())
}

fn looks_like_windows_absolute_path(value: &str) -> bool {
    let bytes = value.as_bytes();

    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

fn validate_extension(category_name: &str, extension: &str) -> Result<(), ConfigValidationError> {
    if extension.trim().is_empty() {
        return Err(ConfigValidationError::new(format!(
            "category '{category_name}' contains an empty extension"
        )));
    }

    if extension.starts_with('.') {
        return Err(ConfigValidationError::new(format!(
            "category '{category_name}' extension '{extension}' must not start with '.'"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        load_config_from_path, validate_config, CategoryRule, Config, ConflictStrategy,
        UnknownFilesBehavior,
    };

    #[test]
    fn loads_valid_config_from_yaml_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("siftforge.yml");

        std::fs::write(
            &config_path,
            r#"
version: 1

behavior:
  unknown_files: skip
  include_hidden: true
  recursive: false
  follow_symlinks: false
  conflict: rename

categories:
  Screenshots:
    extensions:
      - png
      - jpg
    filename_starts_with:
      - Screenshot
"#,
        )
        .unwrap();

        let config = load_config_from_path(&config_path).unwrap();

        assert_eq!(config.version, 1);
        assert_eq!(config.behavior.unknown_files, UnknownFilesBehavior::Skip);
        assert!(config.behavior.include_hidden);
        assert_eq!(config.behavior.conflict, ConflictStrategy::Rename);
        assert!(config.categories.contains_key("Screenshots"));
        assert_eq!(
            config.categories["Screenshots"].extensions,
            vec!["png".to_string(), "jpg".to_string()]
        );
    }

    #[test]
    fn loads_defaults_when_optional_sections_are_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("siftforge.yml");

        std::fs::write(
            &config_path,
            r#"
version: 1
"#,
        )
        .unwrap();

        let config = load_config_from_path(&config_path).unwrap();

        assert_eq!(config.version, 1);
        assert_eq!(config.behavior.unknown_files, UnknownFilesBehavior::Other);
        assert!(!config.behavior.include_hidden);
        assert!(config.categories.is_empty());
    }

    #[test]
    fn returns_error_for_invalid_yaml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("siftforge.yml");

        std::fs::write(&config_path, "version: [").unwrap();

        let error = load_config_from_path(&config_path).unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn validates_valid_config() {
        let mut config = Config::default();

        config.categories.insert(
            "Screenshots".to_string(),
            CategoryRule {
                extensions: vec!["png".to_string()],
                filename_starts_with: Vec::new(),
                filename_contains: Vec::new(),
            },
        );

        validate_config(&config).unwrap();
    }

    #[test]
    fn rejects_unsupported_config_version() {
        let config = Config {
            version: 999,
            ..Default::default()
        };

        let error = validate_config(&config).unwrap_err();

        assert!(error.message.contains("unsupported config version"));
    }

    #[test]
    fn rejects_category_with_path_separator() {
        let mut config = Config::default();

        config.categories.insert(
            "Media/Images".to_string(),
            CategoryRule {
                extensions: vec!["png".to_string()],
                filename_starts_with: Vec::new(),
                filename_contains: Vec::new(),
            },
        );

        let error = validate_config(&config).unwrap_err();

        assert!(error.message.contains("single directory name"));
    }

    #[test]
    fn rejects_category_with_no_match_conditions() {
        let mut config = Config::default();

        config
            .categories
            .insert("Empty".to_string(), CategoryRule::default());

        let error = validate_config(&config).unwrap_err();

        assert!(error.message.contains("no match conditions"));
    }

    #[test]
    fn rejects_extension_starting_with_dot() {
        let mut config = Config::default();

        config.categories.insert(
            "Images".to_string(),
            CategoryRule {
                extensions: vec![".png".to_string()],
                filename_starts_with: Vec::new(),
                filename_contains: Vec::new(),
            },
        );

        let error = validate_config(&config).unwrap_err();

        assert!(error.message.contains("must not start with '.'"));
    }
}
