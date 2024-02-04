use criterion::{criterion_group, criterion_main, Criterion};
use day_15::border_intersection;
use day_15::column_skipping;
use day_15::range_exclusion;
use day_15::range_exclusion::aabb_from_sensor;
use day_15::test_case;
use day_15::test_case::TestCase;

pub fn bench_algorithms(c: &mut Criterion) {
    let inputs = vec![
        TestCase::from(&test_case::AOC_ACTUAL),
        TestCase::from(&test_case::AOC_EXAMPLE),
    ];

    let mut group = c.benchmark_group("Algorithm");
    for input in inputs.iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("00. Brute Force", input.name),
            &input,
            |bencher, input| {
                let mut sensors = input.sensors.clone();
                bencher.iter(|| if input.name != test_case::AOC_ACTUAL.name { day_15::brute_force::solve(&mut sensors, input.dimension); });
            },
        );

        group.bench_with_input(
            criterion::BenchmarkId::new("01. Column Skipping", input.name),
            &input,
            |bencher, input| {
                let mut sensors = input.sensors.clone();
                bencher.iter(|| column_skipping::solve(&mut sensors, input.dimension));
            },
        );

        group.bench_with_input(
            criterion::BenchmarkId::new("02. Range Exclusion", input.name),
            &input,
            |bencher, input| {
                let mut sensors = input
                    .sensors
                    .iter()
                    .map(|s| aabb_from_sensor(s, input.dimension))
                    .collect();
                bencher.iter(|| range_exclusion::solve(&mut sensors, input.dimension));
            },
        );

        group.bench_with_input(
            criterion::BenchmarkId::new("03. Border Intersection", input.name),
            &input,
            |bencher, input| {
                let mut sensors = input.sensors.clone();
                bencher.iter(|| border_intersection::solve(&mut sensors, input.dimension));
            },
        );
    }
}

criterion_group!(benches, bench_algorithms);
criterion_main!(benches);
