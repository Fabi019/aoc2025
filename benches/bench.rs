use criterion::{criterion_group, criterion_main};

// To run individual benchmarks use:
// $ cargo bench --bench bench -- <name>
// where <name> can be like: day07, 07, 07/1, 7/2

aoc2025::benchs!(day01, day02, day03);

criterion_main!(benchmarks);
