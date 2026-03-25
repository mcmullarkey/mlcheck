use regex::Regex;

use crate::domain::types::{
    CheckCategory, CheckResult, Detected, Framework, Language, RuleId, SourceContent,
};

/// How a rule matches against source content.
#[derive(Debug, Clone)]
pub enum MatchStrategy {
    /// Regex pattern applied to code lines only (not comments).
    CodeOnly(Regex),

    /// Any sub-strategy must match (logical OR).
    Any(Vec<MatchStrategy>),

    /// All sub-strategies must match (logical AND).
    All(Vec<MatchStrategy>),
}

/// A check rule definition — pure data.
#[derive(Debug, Clone)]
pub struct CheckRule {
    pub id: RuleId,
    pub category: CheckCategory,
    pub description: String,
    pub language: Language,
    pub framework: Framework,
    pub strategy: MatchStrategy,
}

/// Pure function: does this strategy match the given source content?
#[must_use]
pub fn matches(strategy: &MatchStrategy, content: &SourceContent) -> bool {
    match strategy {
        MatchStrategy::CodeOnly(re) => {
            content.code_lines.iter().any(|line| re.is_match(line))
        }
        MatchStrategy::Any(strategies) => {
            strategies.iter().any(|s| matches(s, content))
        }
        MatchStrategy::All(strategies) => {
            strategies.iter().all(|s| matches(s, content))
        }
    }
}

/// Evaluate a single rule against source content. Pure function.
#[must_use]
pub fn evaluate_rule(rule: &CheckRule, content: &SourceContent) -> CheckResult {
    let detected = if matches(&rule.strategy, content) {
        Detected::Present
    } else {
        Detected::Absent
    };

    CheckResult {
        rule_id: rule.id.clone(),
        category: rule.category,
        description: rule.description.clone(),
        detected,
    }
}

/// Evaluate all rules against source content. Pure function.
#[must_use]
pub fn evaluate_all(rules: &[CheckRule], content: &SourceContent) -> Vec<CheckResult> {
    rules.iter().map(|rule| evaluate_rule(rule, content)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rule(strategy: MatchStrategy) -> CheckRule {
        CheckRule {
            id: RuleId("test-rule".into()),
            category: CheckCategory::LibraryImport,
            description: "Test rule".into(),
            language: Language::Python,
            framework: Framework::ScikitLearn,
            strategy,
        }
    }

    #[test]
    fn code_only_matches_code_line() {
        let content = SourceContent::parse("from sklearn import svm", Language::Python);
        let strategy = MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap());
        assert!(matches(&strategy, &content));
    }

    #[test]
    fn code_only_does_not_match_comment_line() {
        let content = SourceContent::parse("# from sklearn import svm", Language::Python);
        let strategy = MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap());
        assert!(!matches(&strategy, &content));
    }

    #[test]
    fn code_only_does_not_match_empty_content() {
        let content = SourceContent::parse("", Language::Python);
        let strategy = MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap());
        assert!(!matches(&strategy, &content));
    }

    #[test]
    fn any_strategy_matches_when_first_matches() {
        let content = SourceContent::parse("from sklearn import svm", Language::Python);
        let strategy = MatchStrategy::Any(vec![
            MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()),
            MatchStrategy::CodeOnly(Regex::new(r"tensorflow").unwrap()),
        ]);
        assert!(matches(&strategy, &content));
    }

    #[test]
    fn any_strategy_matches_when_second_matches() {
        let content = SourceContent::parse("import tensorflow", Language::Python);
        let strategy = MatchStrategy::Any(vec![
            MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()),
            MatchStrategy::CodeOnly(Regex::new(r"tensorflow").unwrap()),
        ]);
        assert!(matches(&strategy, &content));
    }

    #[test]
    fn any_strategy_does_not_match_when_none_match() {
        let content = SourceContent::parse("import numpy", Language::Python);
        let strategy = MatchStrategy::Any(vec![
            MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()),
            MatchStrategy::CodeOnly(Regex::new(r"tensorflow").unwrap()),
        ]);
        assert!(!matches(&strategy, &content));
    }

    #[test]
    fn all_strategy_matches_when_all_match() {
        let content = SourceContent::parse("from sklearn import svm\nimport numpy", Language::Python);
        let strategy = MatchStrategy::All(vec![
            MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()),
            MatchStrategy::CodeOnly(Regex::new(r"numpy").unwrap()),
        ]);
        assert!(matches(&strategy, &content));
    }

    #[test]
    fn all_strategy_does_not_match_when_one_missing() {
        let content = SourceContent::parse("from sklearn import svm", Language::Python);
        let strategy = MatchStrategy::All(vec![
            MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()),
            MatchStrategy::CodeOnly(Regex::new(r"numpy").unwrap()),
        ]);
        assert!(!matches(&strategy, &content));
    }

    #[test]
    fn evaluate_rule_returns_present_when_matched() {
        let content = SourceContent::parse("from sklearn import svm", Language::Python);
        let rule = make_rule(MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()));
        let result = evaluate_rule(&rule, &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn evaluate_rule_returns_absent_when_not_matched() {
        let content = SourceContent::parse("import numpy", Language::Python);
        let rule = make_rule(MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap()));
        let result = evaluate_rule(&rule, &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn evaluate_all_returns_correct_count() {
        let content = SourceContent::parse("from sklearn import svm", Language::Python);
        let rules = vec![
            make_rule(MatchStrategy::CodeOnly(Regex::new(r"sklearn").unwrap())),
            make_rule(MatchStrategy::CodeOnly(Regex::new(r"tensorflow").unwrap())),
        ];
        let results = evaluate_all(&rules, &content);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].detected, Detected::Present);
        assert_eq!(results[1].detected, Detected::Absent);
    }
}
