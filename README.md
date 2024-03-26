# rust-merkle

A Rust implementation of [Streaming Merkle Roots within Binary Numeral Trees](https://eprint.iacr.org/2021/038.pdf).

## Getting Started

You will need to have Rust and Cargo installed.

```bash
# Build the project
$ cargo build --release

# Run all tests
$ cargo test

# Run the examples
$ cargo run --example merkle_tree
$ cargo run --example merkle_proof
```

## Usage

### Merkle tree example

```bash
$ cargo run --example merkle_tree

# Sample response
Root hash: [62, 46, 173, 253, 186, 61, 168, 141, 75, 120, 173, 174, 83, 231, 152, 174, 239, 123, 191, 145, 195, 31, 237, 163, 145, 241, 68, 178, 111, 252, 114, 152]
Subroot hash: [3, 144, 88, 198, 242, 192, 203, 73, 44, 83, 59, 10, 77, 20, 239, 119, 204, 15, 120, 171, 204, 206, 213, 40, 125, 132, 161, 162, 1, 28, 251, 129]
Subroot hash at index 2: [62, 46, 173, 253, 186, 61, 168, 141, 75, 120, 173, 174, 83, 231, 152, 174, 239, 123, 191, 145, 195, 31, 237, 163, 145, 241, 68, 178, 111, 252, 114, 152]
Limited root hash: [246, 136, 245, 181, 133, 162, 224, 228, 184, 183, 155, 67, 76, 181, 95, 4, 69, 2, 210, 85, 124, 192, 255, 116, 4, 0, 25, 224, 119, 207, 215, 93]
Time elapsed: 100ns
```

### Merkle proof example

```bash
$ cargo run --example merkle_proof

# Sample response
Leaf exists in Merkle tree: true
Computed root: [148, 116, 171, 150, 234, 181, 6, 118, 128, 13, 166, 250, 147, 231, 112, 159, 151, 214, 9, 224, 6, 54, 211, 103, 161, 31, 161, 121, 251, 31, 152, 244]
Known root: [148, 116, 171, 150, 234, 181, 6, 118, 128, 13, 166, 250, 147, 231, 112, 159, 151, 214, 9, 224, 6, 54, 211, 103, 161, 31, 161, 121, 251, 31, 152, 244]
Roots match: true
Time elapsed: 100ns
```

## To-do

Add the remaining proofs from the whitepaper:

- ~~Single-leaf Proofs~~
- Single-range Proofs
- Multi-range Proofs

## References

- Streaming Merkle Proofs within Binary Numeral Trees, by [Luke Champine](https://eprint.iacr.org/2021/038.pdf)

## License

rust-merkle is distributed under the MIT license.
