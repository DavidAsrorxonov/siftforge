#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Images,
    Videos,
    Audio,
    Documents,
    Archives,
    Code,
    Installers,
    Other,
}

impl Category {
    pub fn folder_name(self) -> &'static str {
        match self {
            Category::Images => "Images",
            Category::Videos => "Videos",
            Category::Audio => "Audio",
            Category::Documents => "Documents",
            Category::Archives => "Archives",
            Category::Code => "Code",
            Category::Installers => "Installers",
            Category::Other => "Other",
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

    if lower_name.ends_with("tar.bz2") {
        return Some("tar.bz2".to_string());
    }

    if lower_name.ends_with("tar.xz") {
        return Some("tar.xz".to_string());
    }

    if lower_name.ends_with("tar.zst") {
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

#[cfg(test)]
mod tests {
    use super::detect_extension;

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

    use super::{classify_file_name, Category};

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
}
