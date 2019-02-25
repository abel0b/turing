[![pipeline status](https://gitlab.com/abeliam/turing/badges/master/pipeline.svg)](https://gitlab.com/abeliam/turing/commits/master)
# Turing Machine Simulator

## Usage
```bash
cargo run <filename> [tapes]
```

## Examples
```bash
cargo run examples/palindrome.tm aabbabababbaa
cargo run examples/binary_add.tm 10111 01010010
cargo run examples/unary_add.tm 111 11
cargo run examples/unary_sub.tm 11111 11
```
