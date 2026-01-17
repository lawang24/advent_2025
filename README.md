# Advent of Code 2025

Solutions for Advent of Code 2025 challenges, written in Rust with benchmarking support.

## 📋 Prerequisites

- Rust toolchain
- [hyperfine](https://github.com/sharkdp/hyperfine) for benchmarking

## 🚀 Quick Start

### Setting Up a Day

1. Create your solution files:
   - `src/bin/p1.rs` for Part 1
   - `src/bin/p2.rs` for Part 2
2. Add `/input.txt` file


### Running Benchmarks

From the project root, run:

```bash
./bench.sh <day_folder>
```

For example:
```bash
./bench.sh day2
```

This will:
- Build the release binaries
- Run hyperfine benchmarks on both `p1` and `p2` binaries
- Generate a `benchmarks.md` file in the day's directory with performance results
- Use 15 warmup runs before measuring

## 📁 Project Structure

```
advent_2025/
├── day1/          # Day 1 solution
├── day2/          # Day 2 solution
├── ...
├── day12/         # Day 12 solution
├── bench.sh       # Benchmarking script
```

Each day directory contains:
- `src/bin/p1.rs` - Part 1 solution
- `src/bin/p2.rs` - Part 2 solution
- `input.txt` - Puzzle input (add your own)
- `test.txt` - Test input (from example)
- `benchmarks.md` - Performance benchmarks for that day

## 📊 Benchmarking

The benchmarking script uses [hyperfine](https://github.com/sharkdp/hyperfine) to measure execution time. Results are saved as markdown tables in each day's `benchmarks.md` file.

### Benchmark Results

![Benchmark chart](/benchmark_chart.png)

| Day | P1 (ms) | P2 (ms) |
| --: | ------: | ------: |
|   1 |     1.4 |     1.6 |
|   2 |     2.3 |    68.4 |
|   3 |     1.3 |     1.5 |
|   4 |     1.3 |     2.6 |
|   5 |     1.0 |     0.9 |
|   6 |     1.2 |     1.1 |
|   7 |     1.2 |     1.1 |
|   8 |    15.1 |    15.2 |
|   9 |     1.2 |    11.5 |
|  10 |    17.6 |     9.3 |
|  11 |     1.4 |     1.5 |
|  12 |     1.4 |       — |





