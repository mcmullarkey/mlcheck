use std::path::PathBuf;

/// Language of the source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Python,
    R,
}

/// ML framework within a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
    ScikitLearn,
    Tidymodels,
}

/// Category of best practice being checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckCategory {
    LibraryImport,
    TrainTestSplit,
    Stratification,
    Reproducibility,
    DataLeakagePrevention,
}

/// Detection outcome — not a raw bool, so swapping variants breaks types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Detected {
    Present,
    Absent,
}

impl Detected {
    #[must_use]
    pub fn is_present(self) -> bool {
        match self {
            Detected::Present => true,
            Detected::Absent => false,
        }
    }
}

/// Newtype for rule identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleId(pub String);

/// Result of evaluating one check rule against one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub rule_id: RuleId,
    pub category: CheckCategory,
    pub description: String,
    pub detected: Detected,
}

/// Parsed source content with code/comment separation.
#[derive(Debug, Clone)]
pub struct SourceContent {
    pub code_lines: Vec<String>,
    pub comment_lines: Vec<String>,
    pub all_lines: Vec<String>,
}

impl SourceContent {
    /// Parse raw source text into code and comment lines.
    /// Both Python and R use `#` for comments.
    #[must_use]
    pub fn parse(raw: &str, language: Language) -> Self {
        let all_lines: Vec<String> = raw.lines().map(String::from).collect();

        let comment_prefix = match language {
            Language::Python => "#",
            Language::R => "#",
        };

        let mut code_lines = Vec::new();
        let mut comment_lines = Vec::new();

        for line in &all_lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(comment_prefix) {
                comment_lines.push(line.clone());
            } else {
                code_lines.push(line.clone());
            }
        }

        Self {
            code_lines,
            comment_lines,
            all_lines,
        }
    }
}

/// Score as percentage of checks that passed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score(f64);

impl Score {
    #[must_use]
    pub fn percentage(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn from_ratio(present: usize, total: usize) -> Self {
        if total == 0 {
            return Score(0.0);
        }
        Score(present as f64 / total as f64 * 100.0)
    }
}

/// Metadata about the file being checked.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub language: Language,
    pub framework: Framework,
}

/// Summary of all check results for one file.
#[derive(Debug, Clone)]
pub struct FileReport {
    pub file_info: FileInfo,
    pub results: Vec<CheckResult>,
    pub score: Score,
    pub group_id: String,
    pub timestamp: String,
}

/// Validated scan target.
#[derive(Debug, Clone)]
pub enum ScanTarget {
    SingleFile(PathBuf),
    Directory(PathBuf),
}

/// Output format selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Console,
    Csv,
    Sqlite,
}

/// Supported source file types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFileType {
    PythonScript,
    JupyterNotebook,
    RScript,
    RMarkdown,
}

impl SourceFileType {
    #[must_use]
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "py" => Some(SourceFileType::PythonScript),
            "ipynb" => Some(SourceFileType::JupyterNotebook),
            "R" => Some(SourceFileType::RScript),
            "Rmd" => Some(SourceFileType::RMarkdown),
            _ => None,
        }
    }

    #[must_use]
    pub fn language(self) -> Language {
        match self {
            SourceFileType::PythonScript | SourceFileType::JupyterNotebook => Language::Python,
            SourceFileType::RScript | SourceFileType::RMarkdown => Language::R,
        }
    }

    #[must_use]
    pub fn default_framework(self) -> Framework {
        match self {
            SourceFileType::PythonScript | SourceFileType::JupyterNotebook => Framework::ScikitLearn,
            SourceFileType::RScript | SourceFileType::RMarkdown => Framework::Tidymodels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detected_present_is_present() {
        assert!(Detected::Present.is_present());
    }

    #[test]
    fn detected_absent_is_not_present() {
        assert!(!Detected::Absent.is_present());
    }

    #[test]
    fn source_content_separates_code_from_comments_python() {
        let raw = "import sklearn\n# this is a comment\nx = 1\n";
        let content = SourceContent::parse(raw, Language::Python);
        assert_eq!(content.code_lines, vec!["import sklearn", "x = 1"]);
        assert_eq!(content.comment_lines, vec!["# this is a comment"]);
    }

    #[test]
    fn source_content_empty_lines_are_comments() {
        let raw = "x = 1\n\ny = 2\n";
        let content = SourceContent::parse(raw, Language::Python);
        assert_eq!(content.code_lines, vec!["x = 1", "y = 2"]);
        assert_eq!(content.comment_lines.len(), 1);
    }

    #[test]
    fn source_content_all_lines_preserved() {
        let raw = "a\n# b\nc\n";
        let content = SourceContent::parse(raw, Language::Python);
        assert_eq!(content.all_lines, vec!["a", "# b", "c"]);
    }

    #[test]
    fn score_from_ratio_100_percent() {
        let score = Score::from_ratio(5, 5);
        assert!((score.percentage() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_from_ratio_0_percent() {
        let score = Score::from_ratio(0, 5);
        assert!((score.percentage() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_from_ratio_60_percent() {
        let score = Score::from_ratio(3, 5);
        assert!((score.percentage() - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_from_ratio_zero_total() {
        let score = Score::from_ratio(0, 0);
        assert!((score.percentage() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn source_file_type_from_py_extension() {
        assert_eq!(SourceFileType::from_extension("py"), Some(SourceFileType::PythonScript));
    }

    #[test]
    fn source_file_type_from_r_extension() {
        assert_eq!(SourceFileType::from_extension("R"), Some(SourceFileType::RScript));
    }

    #[test]
    fn source_file_type_from_ipynb_extension() {
        assert_eq!(SourceFileType::from_extension("ipynb"), Some(SourceFileType::JupyterNotebook));
    }

    #[test]
    fn source_file_type_from_rmd_extension() {
        assert_eq!(SourceFileType::from_extension("Rmd"), Some(SourceFileType::RMarkdown));
    }

    #[test]
    fn source_file_type_from_unknown_extension() {
        assert_eq!(SourceFileType::from_extension("txt"), None);
    }

    #[test]
    fn python_script_language_is_python() {
        assert_eq!(SourceFileType::PythonScript.language(), Language::Python);
    }

    #[test]
    fn r_script_language_is_r() {
        assert_eq!(SourceFileType::RScript.language(), Language::R);
    }

    #[test]
    fn python_script_default_framework_is_sklearn() {
        assert_eq!(SourceFileType::PythonScript.default_framework(), Framework::ScikitLearn);
    }

    #[test]
    fn r_script_default_framework_is_tidymodels() {
        assert_eq!(SourceFileType::RScript.default_framework(), Framework::Tidymodels);
    }

    #[test]
    fn jupyter_notebook_language_is_python() {
        assert_eq!(SourceFileType::JupyterNotebook.language(), Language::Python);
    }

    #[test]
    fn rmarkdown_language_is_r() {
        assert_eq!(SourceFileType::RMarkdown.language(), Language::R);
    }

    #[test]
    fn jupyter_notebook_default_framework_is_sklearn() {
        assert_eq!(SourceFileType::JupyterNotebook.default_framework(), Framework::ScikitLearn);
    }

    #[test]
    fn rmarkdown_default_framework_is_tidymodels() {
        assert_eq!(SourceFileType::RMarkdown.default_framework(), Framework::Tidymodels);
    }
}
