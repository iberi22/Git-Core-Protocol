//! Performance benchmarks for Issue Syncer
//!
//! Measures operations against PowerShell baseline (~5-10s for full sync).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use issue_syncer::{
    github::GitHubClient,
    mapping::IssueMapping,
    parser::parse_frontmatter,
    syncer::IssueSyncer,
};
use octocrab::Octocrab;
use std::fs;
use tempfile::TempDir;
use tokio::runtime::Runtime;

fn create_test_syncer(temp_dir: &TempDir) -> IssueSyncer {
    let rt = Runtime::new().unwrap();
    let issues_dir = temp_dir.path().join("issues");
    fs::create_dir_all(&issues_dir).unwrap();

    let mapping_file = issues_dir.join(".issue-mapping.json");

    let github = rt.block_on(async {
        let client = Octocrab::builder().build().unwrap();
        GitHubClient::new(client, "owner".to_string(), "repo".to_string())
    });

    IssueSyncer::new(github, issues_dir, mapping_file)
        .unwrap()
        .with_dry_run(true)
}

fn bench_frontmatter_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("frontmatter_parsing");

    let simple_issue = r#"---
title: "Simple Issue"
labels:
  - bug
---

Body content.
"#;

    let complex_issue = r#"---
title: "Complex Issue with Many Labels"
labels:
  - enhancement
  - high-priority
  - needs-review
  - rust
  - performance
assignees:
  - user1
  - user2
  - user3
---

# Complex Body

This is a complex issue with:
- Multiple sections
- Code blocks
- Lists

```rust
fn main() {
    println!("Hello");
}
```
"#;

    group.bench_function("simple_issue", |b| {
        b.iter(|| {
            let _issue = parse_frontmatter(black_box(simple_issue)).unwrap();
        });
    });

    group.bench_function("complex_issue", |b| {
        b.iter(|| {
            let _issue = parse_frontmatter(black_box(complex_issue)).unwrap();
        });
    });

    group.finish();
}

fn bench_mapping_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("mapping_operations");

    let temp_dir = TempDir::new().unwrap();
    let mapping_file = temp_dir.path().join(".mapping.json");

    group.bench_function("add_mapping", |b| {
        b.iter(|| {
            let mut mapping = IssueMapping::default();
            mapping.add(black_box("FEAT_test.md".to_string()), black_box(42));
        });
    });

    group.bench_function("lookup_by_file", |b| {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);

        b.iter(|| {
            let _issue = mapping.get_issue(black_box("FEAT_test.md"));
        });
    });

    group.bench_function("lookup_by_issue", |b| {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);

        b.iter(|| {
            let _file = mapping.get_file(black_box(42));
        });
    });

    group.bench_function("save_mapping", |b| {
        let mut mapping = IssueMapping::default();
        for i in 0..10 {
            mapping.add(format!("FEAT_{}.md", i), i as u64);
        }

        b.iter(|| {
            mapping.save(black_box(&mapping_file)).unwrap();
        });
    });

    group.bench_function("load_mapping", |b| {
        let mut mapping = IssueMapping::default();
        for i in 0..10 {
            mapping.add(format!("FEAT_{}.md", i), i as u64);
        }
        mapping.save(&mapping_file).unwrap();

        b.iter(|| {
            let _loaded = IssueMapping::load(black_box(&mapping_file)).unwrap();
        });
    });

    group.finish();
}

fn bench_file_scanning(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_scanning");

    // Setup: Create temp directory with multiple issue files
    let temp_dir = TempDir::new().unwrap();
    let syncer = create_test_syncer(&temp_dir);

    let issues_dir = temp_dir.path().join("issues");

    // Create 10 issue files
    for i in 0..10 {
        let content = format!(
            r#"---
title: "Issue {}"
labels:
  - test
---

Body for issue {}.
"#,
            i, i
        );
        fs::write(issues_dir.join(format!("FEAT_{}.md", i)), content).unwrap();
    }

    group.bench_function("scan_10_files", |b| {
        b.iter(|| {
            // Access scan_issue_files through a public method or create new syncer
            let syncer = create_test_syncer(&temp_dir);
            black_box(syncer);
        });
    });

    group.finish();
}

fn bench_dry_run_sync(c: &mut Criterion) {
    let mut group = c.benchmark_group("dry_run_sync");

    let temp_dir = TempDir::new().unwrap();
    let issues_dir = temp_dir.path().join("issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // Create 5 test files
    for i in 0..5 {
        let content = format!(
            r#"---
title: "Test Issue {}"
labels:
  - enhancement
---

Body content {}.
"#,
            i, i
        );
        fs::write(issues_dir.join(format!("FEAT_{}.md", i)), content).unwrap();
    }

    group.bench_function("push_5_files_dry_run", |b| {
        let rt = Runtime::new().unwrap();

        b.iter(|| {
            rt.block_on(async {
                let mut syncer = create_test_syncer(&temp_dir);
                let _report = syncer.push().await.unwrap();
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_frontmatter_parsing,
    bench_mapping_operations,
    bench_file_scanning,
    bench_dry_run_sync,
);
criterion_main!(benches);
