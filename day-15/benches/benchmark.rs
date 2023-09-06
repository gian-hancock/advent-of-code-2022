use criterion::{criterion_group, criterion_main, Criterion};
use day_15::parse;
use day_15::parse_and_solve_by_column_skipping;
use day_15::parse_and_solve_by_range_exclusion;
use day_15::solve_by_border_intersection;
use day_15::solve_by_column_skipping;
use day_15::solve_by_range_exclusion;
use day_15::test_case;

pub fn criterion_benchmark(c: &mut Criterion) {
    // process_file
    c.bench_function("process_file::range_exclusion::aoc_actual", |b| {
        b.iter(|| {
            parse_and_solve_by_range_exclusion(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1)
        })
    });
    c.bench_function("process_file::range_exclusion::aoc_example", |b| {
        b.iter(|| {
            parse_and_solve_by_range_exclusion(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1)
        })
    });
    c.bench_function("process_file::column_skipping::aoc_actual", |b| {
        b.iter(|| {
            parse_and_solve_by_column_skipping(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1)
        })
    });
    c.bench_function("process_file::column_skipping::aoc_example", |b| {
        b.iter(|| {
            parse_and_solve_by_column_skipping(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1)
        })
    });
    // c.bench_function("process_file::border_intersection::aoc_actual", |b| {
    //     b.iter(|| {
    //         parse_and_solve_by_border_intersection(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1)
    //     })
    // });
    // c.bench_function("process_file::border_intersection::aoc_example", |b| {
    //     b.iter(|| {
    //         parse_and_solve_by_border_intersection(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1)
    //     })
    // });

    // process_sensors
    let mut parsed_aoc_actual = parse(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1);
    let mut parsed_aoc_example = parse(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1);
    c.bench_function("exec::range_exclusion::aoc_actual", |b| {
        b.iter(|| solve_by_range_exclusion(&mut parsed_aoc_actual, test_case::AOC_ACTUAL.1))
    });
    c.bench_function("exec::range_exclusion::aoc_example", |b| {
        b.iter(|| solve_by_range_exclusion(&mut parsed_aoc_example, test_case::AOC_EXAMPLE.1))
    });

    let mut parsed_aoc_actual = parse(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1);
    let mut parsed_aoc_example = parse(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1);
    c.bench_function("exec::column_skipping::aoc_actual", |b| {
        b.iter(|| solve_by_column_skipping(&mut parsed_aoc_actual, test_case::AOC_ACTUAL.1))
    });
    c.bench_function("exec::column_skipping::aoc_example", |b| {
        b.iter(|| solve_by_column_skipping(&mut parsed_aoc_example, test_case::AOC_EXAMPLE.1))
    });

    let mut parsed_aoc_actual = parse(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1);
    let mut parsed_aoc_example = parse(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1);
    c.bench_function("exec::border_intersection::aoc_actual", |b| {
        b.iter(|| solve_by_border_intersection(&mut parsed_aoc_actual, test_case::AOC_ACTUAL.1))
    });
    c.bench_function("exec::border_intersection::aoc_example", |b| {
        b.iter(|| solve_by_border_intersection(&mut parsed_aoc_example, test_case::AOC_EXAMPLE.1))
    });
}

pub fn criterion_benchmark_2(c: &mut Criterion) {}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
