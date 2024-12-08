# Advent of Code (Rust Edition)

My tooling for and solutions to [Advent of Code][aoc].

## Development

### Code quality

Check that all code passes:

```bash
cargo clippy --release --all-features --all-targets -- -D warnings
cargo +nightly-2024-03-28 fmt --check
AOC_DATA_DIR=$(pwd)/data cargo test --release --all-features --all-targets
```

[aoc]: https://adventofcode.com/
