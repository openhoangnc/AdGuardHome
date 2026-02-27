//! Criterion benchmarks for the filtering engine — TASK-46.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

use agh_filtering::matcher::FilteringEngine;
use agh_filtering::parser::{FilterRule, parse_filter};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Build a FilteringEngine with N simple block rules (||aN.example^ format).
fn build_engine_with_n_rules(n: usize) -> FilteringEngine {
    let rules: Vec<FilterRule> = (0..n)
        .map(|i| FilterRule::DomainBlock {
            domain: format!("a{i}.example.com"),
        })
        .collect();
    FilteringEngine::build(rules)
}

/// Build a FilteringEngine from the easylist fixture file.
fn build_engine_from_easylist() -> FilteringEngine {
    let content = include_str!("../tests/fixtures/easylist_sample.txt");
    let rules = parse_filter(content);
    FilteringEngine::build(rules)
}

// ── Benchmarks ────────────────────────────────────────────────────────────────

fn bench_engine_build_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_build");
    for &n in &[1_000usize, 10_000, 100_000, 500_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let rules: Vec<FilterRule> = (0..n)
                .map(|i| FilterRule::Block {
                    domain: format!("a{i}.example.com"),
                })
                .collect();
            b.iter(|| {
                FilteringEngine::build(black_box(
                    (0..n)
                        .map(|i| FilterRule::DomainBlock {
                            domain: format!("a{i}.example.com"),
                        })
                        .collect(),
                ))
            })
        });
    }
    group.finish();
}

fn bench_check_domain_no_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_domain_no_match");
    for &n in &[1_000usize, 100_000, 500_000] {
        let engine = build_engine_with_n_rules(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| engine.check_domain(black_box("google.com")))
        });
    }
    group.finish();
}

fn bench_check_domain_blocked(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_domain_blocked");
    for &n in &[1_000usize, 100_000, 500_000] {
        let engine = build_engine_with_n_rules(n);
        // This domain IS in the blocklist.
        let blocked = format!("a{}.example.com", n / 2);
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| engine.check_domain(black_box(&blocked)))
        });
    }
    group.finish();
}

fn bench_check_domain_easylist(c: &mut Criterion) {
    let engine = build_engine_from_easylist();
    c.bench_function("check_domain_easylist_no_match", |b| {
        b.iter(|| engine.check_domain(black_box("google.com")))
    });
    c.bench_function("check_domain_easylist_blocked", |b| {
        b.iter(|| engine.check_domain(black_box("ads.example.com")))
    });
}

fn bench_parse_filter(c: &mut Criterion) {
    let content = include_str!("../tests/fixtures/easylist_sample.txt");
    c.bench_function("parse_easylist_sample", |b| {
        b.iter(|| parse_filter(black_box(content)))
    });
}

criterion_group!(
    benches,
    bench_engine_build_time,
    bench_check_domain_no_match,
    bench_check_domain_blocked,
    bench_check_domain_easylist,
    bench_parse_filter,
);
criterion_main!(benches);
