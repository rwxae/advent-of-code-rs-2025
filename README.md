# Advent of Code 2025

This repository contains my solutions to [AoC 2025](https://adventofcode.com/2025)
implemented in Rust programming language.

## Test

Each solution has a dedicated test module that expects that you have a folder
`inputs` in the root of the project, which contains text files of the input
data for each day. These text files have the following format `{day}-{num}.txt`.

Examples:

```sh
inputs/
  01-01.txt
  01-02.txt
  02-01.txt
  02-02.txt
  ...
```

Once you populate the folder with actual data, you can run tests using `cargo`:

```sh
cargo test
```
