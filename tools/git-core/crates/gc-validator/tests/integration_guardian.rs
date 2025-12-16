//! Integration tests for Guardian Core
//!
//! These tests verify the decision-making logic of Guardian Agent
//! with realistic PR scenarios.

use workflow_orchestrator::guardian_core::{Decision, GuardianCore};
use octocrab::Octocrab;

/// Test helper to create a GuardianCore instance
fn create_guardian() -> GuardianCore {
    let github = Octocrab::builder().build().unwrap();
    GuardianCore::new(github, "owner".to_string(), "repo".to_string())
}

#[tokio::test]
async fn test_confidence_calculation_small_clean_pr() {
    let guardian = create_guardian();

    // Small PR: 50 additions, 30 deletions
    let size_penalty = guardian.calculate_size_penalty(50, 30);
    assert_eq!(size_penalty, 0, "Small PR should have no penalty");

    // Has tests
    let files = vec![
        "src/lib.rs".to_string(),
        "tests/test_lib.rs".to_string(),
    ];
    assert!(guardian.has_tests(&files), "Should detect test files");

    // Different scopes (src/ vs tests/)
    assert!(!guardian.is_single_scope(&files), "Different top-level directories");
}

#[tokio::test]
async fn test_size_penalty_thresholds() {
    let guardian = create_guardian();

    // No penalty (<100 lines)
    assert_eq!(guardian.calculate_size_penalty(50, 50), 0);

    // Small penalty (100-300 lines)
    assert_eq!(guardian.calculate_size_penalty(150, 150), 5);

    // Medium penalty (300-500 lines)
    assert_eq!(guardian.calculate_size_penalty(400, 100), 10);

    // Large penalty (>500 lines)
    assert_eq!(guardian.calculate_size_penalty(600, 600), 20);
}

#[tokio::test]
async fn test_has_tests_detection() {
    let guardian = create_guardian();

    // Has tests (tests/ folder)
    let files_with_tests = vec![
        "src/main.rs".to_string(),
        "tests/test_main.rs".to_string(),
    ];
    assert!(guardian.has_tests(&files_with_tests));

    // Has tests (__tests__/ folder)
    let files_with_jest = vec![
        "src/component.tsx".to_string(),
        "__tests__/component.test.tsx".to_string(),
    ];
    assert!(guardian.has_tests(&files_with_jest));

    // Has tests (.test. suffix)
    let files_with_suffix = vec![
        "src/utils.ts".to_string(),
        "src/utils.test.ts".to_string(),
    ];
    assert!(guardian.has_tests(&files_with_suffix));

    // No tests
    let files_no_tests = vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
    ];
    assert!(!guardian.has_tests(&files_no_tests));
}

#[tokio::test]
async fn test_single_scope_detection() {
    let guardian = create_guardian();

    // Single scope - all in src/
    let single_scope = vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/utils.rs".to_string(),
    ];
    assert!(guardian.is_single_scope(&single_scope));

    // Multiple scopes - src/ and tests/
    let multi_scope = vec![
        "src/main.rs".to_string(),
        "tests/integration.rs".to_string(),
    ];
    assert!(!guardian.is_single_scope(&multi_scope));

    // Multiple scopes - different root folders
    let different_roots = vec![
        "backend/src/main.rs".to_string(),
        "frontend/src/app.tsx".to_string(),
    ];
    assert!(!guardian.is_single_scope(&different_roots));
}

#[tokio::test]
async fn test_decision_from_confidence_thresholds() {
    // Auto-merge when above threshold
    let decision = Decision::from_confidence(80, 70, None);
    assert!(matches!(decision, Decision::AutoMerge { confidence: 80 }));

    // Escalate when below threshold
    let decision = Decision::from_confidence(65, 70, None);
    assert!(matches!(decision, Decision::Escalate { confidence: 65, .. }));

    // Blocked when blocker present (ignores confidence)
    let decision = Decision::from_confidence(90, 70, Some("high-stakes label".to_string()));
    assert!(matches!(decision, Decision::Blocked { .. }));
}

#[tokio::test]
async fn test_realistic_scenario_small_feature() {
    let guardian = create_guardian();

    // Scenario: Small feature with tests
    // - 60 additions, 30 deletions = 90 total lines = 0 size penalty
    // - Has tests = +10 bonus
    // - Single scope = +10 bonus

    let size_penalty = guardian.calculate_size_penalty(60, 30);
    assert_eq!(size_penalty, 0, "90 total lines should have no penalty");

    let files = vec![
        "src/feature.rs".to_string(),
        "src/feature_test.rs".to_string(),  // Keep tests in src/ for single scope
    ];

    assert!(guardian.has_tests(&files), "Feature has tests");
    assert!(guardian.is_single_scope(&files), "Single scope change");

    // With CI passing + review approved, expected confidence:
    // Base: 40 (CI) + 40 (review) = 80
    // Bonus: +10 (tests) + 10 (single scope) = 100
    // Penalty: 0 (small size)
    // Final: 100 (capped at 100)
    // Expected: AutoMerge
}

#[tokio::test]
async fn test_realistic_scenario_large_refactor() {
    let guardian = create_guardian();

    // Scenario: Large refactor across multiple modules
    // - 700 additions, 500 deletions = 20 size penalty
    // - Has tests = +10 bonus
    // - Multiple scopes = 0 bonus

    let size_penalty = guardian.calculate_size_penalty(700, 500);
    assert_eq!(size_penalty, 20, "Large PR should have maximum penalty");

    let files = vec![
        "src/module_a/file1.rs".to_string(),
        "src/module_b/file2.rs".to_string(),
        "src/module_c/file3.rs".to_string(),
        "tests/integration.rs".to_string(),
    ];

    assert!(guardian.has_tests(&files));
    assert!(!guardian.is_single_scope(&files), "Multi-module refactor");

    // Expected confidence:
    // Base: 40 (CI) + 40 (review) = 80
    // Bonus: +10 (tests) = 90
    // Penalty: -20 (large size) = 70
    // Final: 70 (exactly at threshold)
    // Expected: AutoMerge (at threshold) or Escalate (if threshold > 70)
}

#[tokio::test]
async fn test_realistic_scenario_no_tests() {
    let guardian = create_guardian();

    // Scenario: Feature without tests
    // - Small size = 0 penalty
    // - No tests = 0 bonus (missing +10)

    let size_penalty = guardian.calculate_size_penalty(60, 40);
    assert_eq!(size_penalty, 0);

    let files = vec![
        "src/feature.rs".to_string(),
        "src/utils.rs".to_string(),
    ];

    assert!(!guardian.has_tests(&files), "No test coverage");
    assert!(guardian.is_single_scope(&files));

    // Expected confidence:
    // Base: 40 (CI) + 40 (review) = 80
    // Bonus: 0 (no tests) + 10 (single scope) = 90
    // Penalty: 0 = 90
    // Final: 90
    // Expected: AutoMerge (but missing tests is risky)
}
