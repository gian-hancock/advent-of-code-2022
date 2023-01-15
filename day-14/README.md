```bash
# Build and run
$ cargo run --bin part_01
$ cargo run --bin part_02

# Profile
$ cargo build --profile release
$ hyperfine '.\target\release\part_01.exe'
Benchmark 1: .\target\release\part_01.exe
  Time (mean ± σ):       8.6 ms ±   2.2 ms    [User: 0.2 ms, System: 1.0 ms]
  Range (min … max):     6.5 ms …  19.9 ms    189 runs
$ hyperfine '.\target\release\part_02.exe'
Benchmark 1: .\target\release\part_02.exe
  Time (mean ± σ):      89.6 ms ±   3.1 ms    [User: 74.8 ms, System: 2.5 ms]
  Range (min … max):    87.6 ms … 102.1 ms    31 runs
```