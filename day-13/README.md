# Day 13
There are two implementations of part 1, and one of part 2.

## Part 1
`cargo run --bin part_01_naive` runs 10,000 iterations of a non optimized version.
`cargo run --bin part_01_single-pass` runs 10,000 iterations of a faster version.

```shell
$ cargo build --release
   Compiling day-13 v0.1.0 (/home/gian/Workspace/advent-of-code-2022/day-13)
    Finished release [optimized] target(s) in 0.24s
$ time ./target/release/part_01_naive 
53900000

real	0m2.807s
user	0m2.787s
sys	0m0.020s
$ time ./target/release/part_01_single-pass 
53900000

real	0m0.089s
user	0m0.073s
sys	0m0.016s
```

## Part 2
```
$ cargo run --bin part_02
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/part_02`
19261
```