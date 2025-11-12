# Sudoku Solver and Generator

This project is a Sudoku solver and generator implemented in Rust.

## Crates Used

### [Quinn](https://crates.io/crates/quinn)

Quinn is an implementation of the QUIC protocol in Rust. I chose Quinn for its performance, security features, and
active maintenance. It provides a high-level API for building QUIC clients and servers, making it easier to work with
the protocol compared to lower-level libraries.

## Alternatives that could be used

### [Rayon](https://crates.io/crates/rayon)

For data parallelism in Rust, Rayon is a popular choice. It provides a simple and efficient way to parallelize
computations over collections. I preferred to use std::thread for finer control over thread management and to avoid the
overhead of Rayon in scenarios where I needed lightweight threading.