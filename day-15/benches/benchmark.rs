use criterion::{criterion_group, criterion_main, Criterion};
use day_15::Sensor;
use day_15::parse;
// use day_15::parse_and_solve_by_border_intersection;
use day_15::solve_by_border_intersection;
use day_15::solve_by_column_skipping;
use day_15::range_exclusion::solve;
use day_15::test_case;

pub fn bench_algorithms(c: &mut Criterion) {
    struct Input {
        name: &'static str,
        sensors: Vec<Sensor>,
        size: i32,
    }
    let inputs = [
        Input {
            name: "AOC Actual",
            sensors: parse(test_case::AOC_ACTUAL.0),
            size: test_case::AOC_ACTUAL.1,
        },
        Input {
            name: "AOC Example",
            sensors: parse(test_case::AOC_EXAMPLE.0),
            size: test_case::AOC_EXAMPLE.1,
        },
    ];

    let mut group = c.benchmark_group("Algorithm");
    for input in inputs.iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("01. Column Skipping", input.name),
            &input,
            |bencher, input| { 
                let mut sensors = input.sensors.clone();
                bencher.iter(|| solve_by_column_skipping(&mut sensors, input.size)); 
            }
        );

        group.bench_with_input(
            criterion::BenchmarkId::new("02. Range Exclusion", input.name),
            &input,
            |bencher, input| { 
                let mut sensors = input.sensors.clone();
                bencher.iter(|| solve(&mut sensors, input.size)); 
            }
        );

        group.bench_with_input(
            criterion::BenchmarkId::new("03. Border Intersection", input.name),
            &input,
            |bencher, input| { 
                let mut sensors = input.sensors.clone();
                bencher.iter(|| solve_by_border_intersection(&mut sensors, input.size)); 
            }
        );
    }
}

criterion_group!(benches, bench_algorithms);
criterion_main!(benches);