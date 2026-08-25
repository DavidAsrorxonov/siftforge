use crate::config::Config;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    Images,
    Videos,
    Audio,
    Documents,
    Archives,
    Code,
    Installers,
    Other,
    Custom(String),
}

impl Category {
    pub fn folder_name(&self) -> String {
        match self {
            Category::Images => "Images".to_string(),
            Category::Videos => "Videos".to_string(),
            Category::Audio => "Audio".to_string(),
            Category::Documents => "Documents".to_string(),
            Category::Archives => "Archives".to_string(),
            Category::Code => "Code".to_string(),
            Category::Installers => "Installers".to_string(),
            Category::Other => "Other".to_string(),
            Category::Custom(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    pub category: Category,
    pub matched_extension: Option<String>,
}

impl Classification {
    pub fn new(category: Category, matched_extension: Option<String>) -> Self {
        Self {
            category,
            matched_extension,
        }
    }
}

pub fn detect_extension(file_name: &str) -> Option<String> {
    let lower_name = file_name.to_lowercase();

    if lower_name.ends_with(".tar.gz") {
        return Some("tar.gz".to_string());
    }

    if lower_name.ends_with(".tar.bz2") {
        return Some("tar.bz2".to_string());
    }

    if lower_name.ends_with(".tar.xz") {
        return Some("tar.xz".to_string());
    }

    if lower_name.ends_with(".tar.zst") {
        return Some("tar.zst".to_string());
    }

    let (_, extension) = lower_name.rsplit_once('.')?;

    if extension.is_empty() {
        None
    } else {
        Some(extension.to_string())
    }
}

pub fn classify_file_name(file_name: &str) -> Classification {
    let extension = detect_extension(file_name);

    let category = match extension.as_deref() {
        Some(
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "heic" | "heif" | "bmp" | "tif"
            | "tiff" | "ico" | "avif",
        ) => Category::Images,

        Some("mp4" | "mov" | "mkv" | "avi" | "webm" | "m4v" | "mpeg" | "mpg" | "wmv" | "flv") => {
            Category::Videos
        }

        Some("mp3" | "wav" | "flac" | "aac" | "m4a" | "ogg" | "opus" | "wma" | "aiff") => {
            Category::Audio
        }

        Some(
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "pages" | "md" | "epub" | "mobi"
            | "xls" | "xlsx" | "csv" | "ods" | "numbers" | "ppt" | "pptx" | "odp" | "key",
        ) => Category::Documents,

        Some(
            "zip" | "rar" | "7z" | "tar" | "gz" | "tgz" | "bz2" | "xz" | "zst" | "tar.gz"
            | "tar.bz2" | "tar.xz" | "tar.zst",
        ) => Category::Archives,

        Some(
            "js" | "mjs" | "cjs" | "ts" | "mts" | "cts" | "jsx" | "tsx" | "py" | "go" | "rs"
            | "java" | "kt" | "kts" | "c" | "h" | "cpp" | "hpp" | "cs" | "php" | "rb" | "swift"
            | "scala" | "sh" | "bash" | "zsh" | "fish" | "ps1" | "html" | "htm" | "css" | "scss"
            | "sass" | "less" | "vue" | "svelte" | "sql" | "graphql" | "gql" | "yaml" | "yml"
            | "toml" | "xml" | "json",
        ) => Category::Code,

        Some("dmg" | "pkg" | "exe" | "msi" | "msp" | "deb" | "rpm" | "appimage" | "apk") => {
            Category::Installers
        }

        _ => Category::Other,
    };

    Classification::new(category, extension)
}

pub fn classify_file_name_with_config(file_name: &str, config: &Config) -> Classification {
    for (category_name, rule) in &config.categories {
        if rule
            .filename_starts_with
            .iter()
            .any(|prefix| file_name.starts_with(prefix))
        {
            return Classification::new(Category::Custom(category_name.clone()), None);
        }
    }

    for (category_name, rule) in &config.categories {
        if rule
            .filename_contains
            .iter()
            .any(|needle| file_name.contains(needle))
        {
            return Classification::new(Category::Custom(category_name.clone()), None);
        }
    }

    let extension = detect_extension(file_name);

    if let Some(extension) = extension.as_deref() {
        for (category_name, rule) in &config.categories {
            if rule
                .extensions
                .iter()
                .any(|configured_extension| configured_extension.eq_ignore_ascii_case(extension))
            {
                return Classification::new(
                    Category::Custom(category_name.clone()),
                    Some(extension.to_string()),
                );
            }
        }
    }

    classify_file_name(file_name)
}

#[cfg(test)]
mod tests {
    use super::detect_extension;
    use crate::config::{CategoryRule, Config};

    #[test]
    fn detects_simple_extension() {
        assert_eq!(detect_extension("report.PDF"), Some("pdf".to_string()))
    }

    #[test]
    fn detects_compound_archive_extension() {
        assert_eq!(
            detect_extension("backup.tar.gz"),
            Some("tar.gz".to_string())
        )
    }

    #[test]
    fn returns_none_for_file_without_extension() {
        assert_eq!(detect_extension("README"), None);
    }

    #[test]
    fn returns_none_for_empty_extension() {
        assert_eq!(detect_extension("filename."), None);
    }

    use super::{classify_file_name, classify_file_name_with_config, Category};

    #[test]
    fn classifies_image_file() {
        let classification = classify_file_name("photo.PNG");

        assert_eq!(classification.category, Category::Images);
        assert_eq!(classification.matched_extension, Some("png".to_string()));
    }

    #[test]
    fn classifies_document_file() {
        let classification = classify_file_name("notes.md");

        assert_eq!(classification.category, Category::Documents);
        assert_eq!(classification.matched_extension, Some("md".to_string()));
    }

    #[test]
    fn classifies_compound_archive_file() {
        let classification = classify_file_name("backup.tar.gz");

        assert_eq!(classification.category, Category::Archives);
        assert_eq!(classification.matched_extension, Some("tar.gz".to_string()));
    }

    #[test]
    fn classifies_unknown_file_as_other() {
        let classification = classify_file_name("mystery.unknownext");

        assert_eq!(classification.category, Category::Other);
        assert_eq!(
            classification.matched_extension,
            Some("unknownext".to_string())
        );
    }

    #[test]
    fn classifies_file_without_extension_as_other() {
        let classification = classify_file_name("README");

        assert_eq!(classification.category, Category::Other);
        assert_eq!(classification.matched_extension, None);
    }

    #[test]
    fn user_extension_rule_overrides_builtin_category() {
        let mut config = Config::default();

        config.categories.insert(
            "University".to_string(),
            CategoryRule {
                extensions: vec!["pdf".to_string()],
                filename_starts_with: Vec::new(),
                filename_contains: Vec::new(),
            },
        );

        let classification = classify_file_name_with_config("report.pdf", &config);

        assert_eq!(
            classification.category,
            Category::Custom("University".to_string())
        );
        assert_eq!(classification.matched_extension, Some("pdf".to_string()));
    }

    #[test]
    fn falls_back_to_builtin_category_when_no_user_rule_matches() {
        let config = Config::default();

        let classification = classify_file_name_with_config("photo.png", &config);

        assert_eq!(classification.category, Category::Images);
    }

    #[test]
    fn user_filename_starts_with_rule_overrides_extension_rule() {
        let mut config = Config::default();

        config.categories.insert(
            "Screenshots".to_string(),
            CategoryRule {
                extensions: Vec::new(),
                filename_starts_with: vec!["Screenshot".to_string()],
                filename_contains: Vec::new(),
            },
        );

        config.categories.insert(
            "ImagesCustom".to_string(),
            CategoryRule {
                extensions: vec!["png".to_string()],
                filename_starts_with: Vec::new(),
                filename_contains: Vec::new(),
            },
        );

        let classification = classify_file_name_with_config("Screenshot 2026-08-25.png", &config);

        assert_eq!(
            classification.category,
            Category::Custom("Screenshots".to_string())
        );
    }

    #[test]
    fn user_filename_contains_rule_overrides_extension_rule() {
        let mut config = Config::default();

        config.categories.insert(
            "University".to_string(),
            CategoryRule {
                extensions: Vec::new(),
                filename_starts_with: Vec::new(),
                filename_contains: vec!["assignment".to_string()],
            },
        );

        config.categories.insert(
            "DocumentsCustom".to_string(),
            CategoryRule {
                extensions: vec!["pdf".to_string()],
                filename_starts_with: Vec::new(),
                filename_contains: Vec::new(),
            },
        );

        let classification = classify_file_name_with_config("math-assignment.pdf", &config);

        assert_eq!(
            classification.category,
            Category::Custom("University".to_string())
        );
    }

    #[test]
    fn user_extension_rule_is_used_when_filename_rules_do_not_match() {
        let mut config = Config::default();

        config.categories.insert(
            "University".to_string(),
            CategoryRule {
                extensions: vec!["pdf".to_string()],
                filename_starts_with: vec!["Lecture".to_string()],
                filename_contains: vec!["assignment".to_string()],
            },
        );

        let classification = classify_file_name_with_config("syllabus.pdf", &config);

        assert_eq!(
            classification.category,
            Category::Custom("University".to_string())
        );
    }
}
