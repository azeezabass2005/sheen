use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarkdownPath(PathBuf); // field is private: can't be built outside this module

impl MarkdownPath {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();
        if !path.exists() {
            return Err(format!("'{}' does not exists", path.display()));
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") => {
                Ok(Self(path))
            }
            _ => Err(format!(
                "'{}' is not a markdown file (.md or .markdown)",
                path.display()
            )),
        }
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl FromStr for MarkdownPath {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<Path> for MarkdownPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl fmt::Display for MarkdownPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}
