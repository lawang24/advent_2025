# Advent of Code 2025

Solutions for Advent of Code 2025 challenges, written in Rust with benchmarking support.

## 📋 Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- [hyperfine](https://github.com/sharkdp/hyperfine) for benchmarking

## 🚀 Quick Start

### Setting Up a Day

1. Create your solution files:
   - `src/bin/p1.rs` for Part 1
   - `src/bin/p2.rs` for Part 2

2. Build the release binaries:
   ```bash
   cd day<N>
   cargo build --release
   ```

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
- Run hyperfine benchmarks on both `p1` and `p2` binaries
- Generate a `benchmarks.md` file in the day's directory with performance results
- Use 15 warmup runs before measuring

## 📁 Project Structure

```
advent_2025/
├── day1/          # Day 1 solution
├── day2/          # Day 2 solution
├── day3/          # Day 3 solution
├── bench.sh       # Benchmarking script
└── benchmarks.md  # Overall benchmark results
```

Each day directory contains:
- `src/bin/p1.rs` - Part 1 solution
- `src/bin/p2.rs` - Part 2 solution
- `input.txt` - Puzzle input (hidden)
- `test.txt` - Test input (if applicable)
- `benchmarks.md` - Performance benchmarks for that day

## 📊 Benchmarking

The benchmarking script uses [hyperfine](https://github.com/sharkdp/hyperfine) to measure execution time. Results are saved as markdown tables in each day's `benchmarks.md` file.

**Why `--release`?** The `--release` flag enables optimizations that can make Rust code run 10-100x faster than debug builds. Debug builds (`cargo build`) include:
- No optimizations (faster compile time, slower runtime)
- Debug symbols and assertions
- Overflow checks



