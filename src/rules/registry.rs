use crate::domain::types::{Framework, Language};
use crate::rules::pattern::CheckRule;
use crate::rules::python::scikit_learn_rules;
use crate::rules::r_lang::tidymodels_rules;

/// Returns all check rules for a given language and framework. Pure function.
#[must_use]
pub fn rules_for(language: Language, framework: Framework) -> Vec<CheckRule> {
    match (language, framework) {
        (Language::Python, Framework::ScikitLearn) => scikit_learn_rules(),
        (Language::R, Framework::Tidymodels) => tidymodels_rules(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_sklearn_returns_five_rules() {
        let rules = rules_for(Language::Python, Framework::ScikitLearn);
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn r_tidymodels_returns_five_rules() {
        let rules = rules_for(Language::R, Framework::Tidymodels);
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn unsupported_combination_returns_empty() {
        let rules = rules_for(Language::R, Framework::ScikitLearn);
        assert!(rules.is_empty());
    }
}
