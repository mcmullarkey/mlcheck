use regex::Regex;

use crate::domain::types::{CheckCategory, Framework, Language, RuleId};
use crate::rules::pattern::{CheckRule, MatchStrategy};

/// Returns all tidymodels best practice check rules.
#[must_use]
pub fn tidymodels_rules() -> Vec<CheckRule> {
    vec![
        CheckRule {
            id: RuleId("r-tidymodels-import".into()),
            category: CheckCategory::LibraryImport,
            description: "Imports the tidymodels library".into(),
            language: Language::R,
            framework: Framework::Tidymodels,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"library\(tidymodels\)").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("r-tidymodels-split".into()),
            category: CheckCategory::TrainTestSplit,
            description: "Splits data into train and test sets".into(),
            language: Language::R,
            framework: Framework::Tidymodels,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"initial_split\s*\(").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("r-tidymodels-strata".into()),
            category: CheckCategory::Stratification,
            description: "Ensures class balance in train/test split".into(),
            language: Language::R,
            framework: Framework::Tidymodels,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"strata\s*=").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("r-tidymodels-seed".into()),
            category: CheckCategory::Reproducibility,
            description: "Sets seed for reproducible results".into(),
            language: Language::R,
            framework: Framework::Tidymodels,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"set\.seed\s*\(").unwrap(),
            ),
        },
        CheckRule {
            id: RuleId("r-tidymodels-recipe".into()),
            category: CheckCategory::DataLeakagePrevention,
            description: "Uses a recipe to guard against data leakage".into(),
            language: Language::R,
            framework: Framework::Tidymodels,
            strategy: MatchStrategy::CodeOnly(
                Regex::new(r"recipe\s*\(").unwrap(),
            ),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::{Detected, Language, SourceContent};
    use crate::rules::pattern::evaluate_rule;

    #[test]
    fn tidymodels_import_detected() {
        let content = SourceContent::parse("library(tidymodels)", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn tidymodels_import_absent() {
        let content = SourceContent::parse("library(dplyr)", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn tidymodels_import_not_detected_in_comment() {
        let content = SourceContent::parse("# library(tidymodels)", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[0], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn initial_split_detected() {
        let content = SourceContent::parse(
            "data_split <- initial_split(data, prop = 0.75)",
            Language::R,
        );
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[1], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn initial_split_absent() {
        let content = SourceContent::parse("model <- fit(data)", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[1], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn strata_detected() {
        let content = SourceContent::parse(
            "initial_split(data, strata = target)",
            Language::R,
        );
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[2], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn strata_absent() {
        let content = SourceContent::parse(
            "initial_split(data, prop = 0.75)",
            Language::R,
        );
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[2], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn set_seed_detected() {
        let content = SourceContent::parse("set.seed(123)", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[3], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn set_seed_absent() {
        let content = SourceContent::parse("x <- 1", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[3], &content);
        assert_eq!(result.detected, Detected::Absent);
    }

    #[test]
    fn recipe_detected() {
        let content = SourceContent::parse(
            "data_recipe <- recipe(target ~ ., data = train_data)",
            Language::R,
        );
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[4], &content);
        assert_eq!(result.detected, Detected::Present);
    }

    #[test]
    fn recipe_absent() {
        let content = SourceContent::parse("model <- fit(data)", Language::R);
        let rules = tidymodels_rules();
        let result = evaluate_rule(&rules[4], &content);
        assert_eq!(result.detected, Detected::Absent);
    }
}
