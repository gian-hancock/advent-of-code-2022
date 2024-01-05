# Advent of Code 2023 - Day 15: Beacon Exclusion Zone
Solutions for [Day 15](https://adventofcode.com/2020/day/15) of the Advent of Code challenge.

## Structure
A single Cargo package containing shared code and binaries for each solution.

### Running the binaries
There are multible binaries in this package, one for part 1 `bin/part_1.rs` and 3 different approaches to solve part 2:
- `bin/part_2_border_intersection.rs`
- `bin/part_2_column_skipping.rs`
- `bin/part_2_range_exclusion.rs`

```bash
cargo run --bin <binary> # Hint: Run `cargo run` to get a list of all available binaries
```

## Benchmarks
There are [Criterion.rs]([day-15\src\bin\part_2_range_exclusion.rs](https://github.com/bheisler/criterion.rs) benchmarks for all 3 part 2 solutions. Run them using `cargo bench`. The results are stored in `/target/criterion/report/index.html`.

## TODO:
- [ ] Find failing test case
- [ ] Rename `dimension` to search_area_width