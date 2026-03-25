use regex::Regex;

use crate::domain::types::{CheckCategory, Framework, Language, RuleId};
use crate::rules::pattern::{CheckRule, MatchStrategy};

/// Returns all scikit-learn best practice check rules.
#[must_use]
pub fn scikit_learn_rules() -> Vec<CheckRule> {
    vec![
        CheckRule {
            id: RuleId("py-sklearn-import".into()),
            category: CheckCategory::LibraryImport,
            description: "Imports the scikit-learn library".into(),
            language: Language::Python,
            framework: Framework::ScikitLearn,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"(?:import|from)\s+sklearn").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("py-sklearn-split".into()),
            category: CheckCategory::TrainTestSplit,
            description: "Splits data into train and test sets".into(),
            language: Language::Python,
            framework: Framework::ScikitLearn,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"train_test_split\s*\(").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("py-sklearn-stratify".into()),
            category: CheckCategory::Stratification,
            description: "Ensures class balance in train/test split".into(),
            language: Language::Python,
            framework: Framework::ScikitLearn,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"stratify\s*=").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("py-sklearn-seed".into()),
            category: CheckCategory::Reproducibility,
            description: "Sets seed for reproducible results".into(),
            language: Language::Python,
            framework: Framework::ScikitLearn,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"random_state\s*=").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("py-sklearn-pipeline".into()),
            category: CheckCategory::DataLeakagePrevention,
            description: "Uses a pipeline to guard against data leakage".into(),
            language: Language::Python,
            framework: Framework::ScikitLearn,
            strategy: MatchStrategy::Any(vec![
                MatchStrategy::CodeOnly(Regex::new(r"Pipeline\s*\(").unwrap()),
                MatchStrategy::CodeOnly(Regex::new(r"make_pipeline\s*\(").unwrap()),
            ]),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::SourceContent;
    use crate::rules::pattern::evaluate_rule;
    use crate::domain::types::Detected;

    #[test]
    fn sklearn_import_detected_with_from_import() {
        let content = SourceContent::parse(
            "from sklearn.model_selection import train_test_split",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn sklearn_import_detected_with_import() {
        let content = SourceContent::parse("import sklearn", Language::Python);
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn sklearn_import_absent_when_not_imported() {
        let content = SourceContent::parse("import numpy", Language::Python);
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn train_test_split_detected() {
        let content = SourceContent::parse(
            "X_train, X_test = train_test_split(X, y)",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[1], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn train_test_split_absent() {
        let content = SourceContent::parse("model.fit(X, y)", Language::Python);
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[1], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn stratify_detected() {
        let content = SourceContent::parse(
            "train_test_split(X, y, stratify=y)",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[2], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn stratify_absent() {
        let content = SourceContent::parse(
            "train_test_split(X, y)",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[2], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn random_state_detected() {
        let content = SourceContent::parse(
            "train_test_split(X, y, random_state=42)",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[3], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn random_state_absent() {
        let content = SourceContent::parse(
            "train_test_split(X, y)",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[3], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn pipeline_detected_with_pipeline() {
        let content = SourceContent::parse(
            "pipe = Pipeline([('scaler', StandardScaler())])",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[4], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn pipeline_detected_with_make_pipeline() {
        let content = SourceContent::parse(
            "pipe = make_pipeline(StandardScaler(), LogisticRegression())",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[4], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn pipeline_absent() {
        let content = SourceContent::parse(
            "model = LinearRegression().fit(X, y)",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[4], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn sklearn_import_not_detected_in_comment() {
        let content = SourceContent::parse(
            "# from sklearn import svm",
            Language::Python,
        );
        let rules = scikit_learn_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Absent);
    }
}
