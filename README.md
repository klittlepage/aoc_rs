# Advent of Code (Rust Edition)

My tooling for and solutions to [Advent of Code][aoc].

## Development

### Downloading AOC Imput Files

```bash
❯ cargo run --release -p aoc_downloader -- --help
Advent Of Code (AOC) tooling and solutions

Usage: aoc_downloader [OPTIONS] <DAY> <PROBLEM_PART>

Arguments:
  <DAY>           Problem day
  <PROBLEM_PART>  Problem part [possible values: p1, p2]

Options:
  -d, --data-dir <DATA_DIR>        Data directory [default: ./data]
  -y, --year <YEAR>                AOC challenge year [default: 2024]
  -a, --aoc-session <AOC_SESSION>  AOC session cookie
  -h, --help                       Print help
  -V, --version                    Print version
```

### Running a Problem or Example

```bash
❯ cargo run --release -p aoc_2024 -- --help
   Compiling cli v0.0.1 (/Users/klittlepage/Documents/personal/src/aoc/lib/cli)
   Compiling aoc_2024 v0.0.1 (/Users/klittlepage/Documents/personal/src/aoc/years/aoc_2024)
    Finished `release` profile [optimized] target(s) in 0.67s
     Running `target/release/aoc_2024 --help`
Advent Of Code (AOC) tooling and solutions

Usage: aoc_2024 [OPTIONS] <DAY> <PROBLEM_PART>

Arguments:
  <DAY>           Problem day
  <PROBLEM_PART>  Problem part [possible values: p1, p2]

Options:
  -d, --data-dir <DATA_DIR>  Data directory [default: ./data]
  -e, --example              Run example
  -h, --help                 Print help
  -V, --version              Print version
```

### Code quality

Check that all code passes:

```bash
cargo clippy --release --all-features --all-targets -- -D warnings
cargo +nightly-2024-03-28 fmt --check
AOC_DATA_DIR=$(pwd)/data cargo test --release --all-features --all-targets
```

Note that `AOC_DATA_DIR` must be set to an absolute path when running tests.

[aoc]: https://adventofcode.com/
