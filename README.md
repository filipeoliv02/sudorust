# sudorust

A small Rust Sudoku library and command-line example supporting square sizes whose sides are perfect squares, including 4×4 and 9×9 boards.

## Algorithms

- **Validation:** Checks board dimensions and every row, column, and square box for duplicate values.
- **Solving:** Uses recursive backtracking. Before each branch, the solver selects the empty cell with the Minimum Remaining Values (MRV); bitmasks represented as `Vec<bool>` candidate lists restrict values to those unused in the cell's row, column, and box. The public all-solutions method and parallel method use this algorithm.
- **Generation:** Adds random clues while checking placements, with a bounded attempt limit for both retries and clues. Generation returns a `BoardError::GenerationFailed` instead of recursing indefinitely.
- **Parsing:** Files contain a first-line size followed by exactly `size` rows of `size` whitespace-separated values. Zero denotes an empty cell.

## Build

```sh
cargo build --locked
```

## Test

```sh
cargo test --locked
cargo clippy --locked -- -D warnings
cargo fmt --check
```

Run the example with:

```sh
cargo run --locked
```
