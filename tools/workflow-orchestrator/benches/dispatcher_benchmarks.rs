//! Performance benchmarks for Dispatcher Core
//!
//! These benchmarks measure the performance-critical paths in the
//! dispatcher agent compared to the PowerShell baseline.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use workflow_orchestrator::dispatcher_core::{Strategy, Agent};

/// Benchmark: Strategy parsing from strings
fn bench_strategy_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("strategy_parsing");
    
    group.bench_function("round_robin", |b| {
        b.iter(|| {
            let _strategy: Strategy = black_box("round-robin").parse().unwrap();
        });
    });
    
    group.bench_function("random", |b| {
        b.iter(|| {
            let _strategy: Strategy = black_box("random").parse().unwrap();
        });
    });
    
    group.bench_function("copilot_only", |b| {
        b.iter(|| {
            let _strategy: Strategy = black_box("copilot-only").parse().unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark: Agent label generation
fn bench_agent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("agent_operations");
    
    group.bench_function("copilot_label", |b| {
        b.iter(|| {
            let label = black_box(Agent::Copilot).label();
            black_box(label);
        });
    });
    
    group.bench_function("jules_label", |b| {
        b.iter(|| {
            let label = black_box(Agent::Jules).label();
            black_box(label);
        });
    });
    
    group.bench_function("copilot_assignee", |b| {
        b.iter(|| {
            let assignee = black_box(Agent::Copilot).assignee();
            black_box(assignee);
        });
    });
    
    group.bench_function("jules_assignee", |b| {
        b.iter(|| {
            let assignee = black_box(Agent::Jules).assignee();
            black_box(assignee);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_strategy_parsing,
    bench_agent_operations
);
criterion_main!(benches);
