use crate::domain::types::{CheckResult, Detected, Score};

/// Calculate score from check results. Pure function.
#[must_use]
pub fn score_from_results(results: &[CheckResult]) -> Score {
    let present = results.iter().filter(|r| r.detected == Detected::Present).count();
    Score::from_ratio(present, results.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::{CheckCategory, RuleId};

    fn make_result(detected: Detected) -> CheckResult {
        CheckResult {
            rule_id: RuleId("test".into()),
            category: CheckCategory::LibraryImport,
            description: "test".into(),
            detected,
        }
    }

    #[test]
    fn score_all_present() {
        let results = vec![
            make_result(Detected::Present),
            make_result(Detected::Present),
        ];
        let score = score_from_results(&results);
        assert!((score.percentage() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_all_absent() {
        let results = vec![
            make_result(Detected::Absent),
            make_result(Detected::Absent),
        ];
        let score = score_from_results(&results);
        assert!((score.percentage() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_mixed() {
        let results = vec![
            make_result(Detected::Present),
            make_result(Detected::Present),
            make_result(Detected::Present),
            make_result(Detected::Absent),
            make_result(Detected::Absent),
        ];
        let score = score_from_results(&results);
        assert!((score.percentage() - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_empty_results() {
        let results: Vec<CheckResult> = vec![];
        let score = score_from_results(&results);
        assert!((score.percentage() - 0.0).abs() < f64::EPSILON);
    }
}
