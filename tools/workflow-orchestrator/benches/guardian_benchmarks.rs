//! Benchmarks for Guardian Core
//! 
//! Compares Rust implementation performance against PowerShell baseline

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use workflow_orchestrator::guardian_core::{GuardianCore, Decision};
use octocrab::Octocrab;
use tokio::runtime::Runtime;

fn create_guardian() -> GuardianCore {
    // Create runtime for Octocrab initialization
    let rt = Runtime::new().unwrap();
    let github = rt.block_on(async {
        Octocrab::builder().build().unwrap()
    });
    GuardianCore::new(github, "owner".to_string(), "repo".to_string())
}

/// Benchmark: Size penalty calculation
fn bench_size_penalty(c: &mut Criterion) {
    let guardian = create_guardian();
    
    let mut group = c.benchmark_group("size_penalty");
    
    for (additions, deletions) in [
        (50, 50),      // Small PR
        (200, 100),    // Medium PR
        (600, 400),    // Large PR
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}+{}", additions, deletions)),
            &(additions, deletions),
            |b, &(add, del)| {
                b.iter(|| guardian.calculate_size_penalty(black_box(add), black_box(del)))
            },
        );
    }
    
    group.finish();
}

/// Benchmark: Has tests detection
fn bench_has_tests(c: &mut Criterion) {
    let guardian = create_guardian();
    
    let test_cases = vec![
        (
            "small_with_tests",
            vec!["src/main.rs".to_string(), "tests/test_main.rs".to_string()],
        ),
        (
            "large_with_tests",
            vec![
                "src/file1.rs".to_string(),
                "src/file2.rs".to_string(),
                "src/file3.rs".to_string(),
                "tests/test1.rs".to_string(),
                "tests/test2.rs".to_string(),
            ],
        ),
        (
            "no_tests",
            vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        ),
    ];
    
    let mut group = c.benchmark_group("has_tests");
    
    for (name, files) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &files, |b, files| {
            b.iter(|| guardian.has_tests(black_box(files)))
        });
    }
    
    group.finish();
}

/// Benchmark: Single scope detection
fn bench_single_scope(c: &mut Criterion) {
    let guardian = create_guardian();
    
    let test_cases = vec![
        (
            "single_scope",
            vec!["src/file1.rs".to_string(), "src/file2.rs".to_string()],
        ),
        (
            "multi_scope",
            vec![
                "src/file1.rs".to_string(),
                "tests/test.rs".to_string(),
                "docs/README.md".to_string(),
            ],
        ),
    ];
    
    let mut group = c.benchmark_group("single_scope");
    
    for (name, files) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &files, |b, files| {
            b.iter(|| guardian.is_single_scope(black_box(files)))
        });
    }
    
    group.finish();
}

/// Benchmark: Decision from confidence
fn bench_decision_from_confidence(c: &mut Criterion) {
    let mut group = c.benchmark_group("decision");
    
    for confidence in [50, 70, 90] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("conf_{}", confidence)),
            &confidence,
            |b, &conf| {
                b.iter(|| Decision::from_confidence(black_box(conf), black_box(70), None))
            },
        );
    }
    
    group.finish();
}

/// Benchmark: Full confidence calculation (simulated)
fn bench_full_confidence_calc(c: &mut Criterion) {
    let guardian = create_guardian();
    
    c.bench_function("full_confidence_simulation", |b| {
        b.iter(|| {
            // Simulate full confidence calculation
            let ci_ok = black_box(true);
            let reviews_ok = black_box(true);
            let size_penalty = guardian.calculate_size_penalty(black_box(150), black_box(100));
            let files = black_box(vec![
                "src/main.rs".to_string(),
                "tests/test.rs".to_string(),
            ]);
            let has_tests = guardian.has_tests(&files);
            let single_scope = guardian.is_single_scope(&files);
            
            let mut confidence = 0u8;
            if ci_ok {
                confidence += 40;
            }
            if reviews_ok {
                confidence += 40;
            }
            if has_tests {
                confidence += 10;
            }
            if single_scope {
                confidence += 10;
            }
            confidence = confidence.saturating_sub(size_penalty);
            
            Decision::from_confidence(confidence, 70, None)
        })
    });
}

criterion_group!(
    benches,
    bench_size_penalty,
    bench_has_tests,
    bench_single_scope,
    bench_decision_from_confidence,
    bench_full_confidence_calc
);

criterion_main!(benches);
