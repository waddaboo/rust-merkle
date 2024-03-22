# rust-merkle

A Rust implementation of [Streaming Merkle Roots within Binary Numeral Trees](https://eprint.iacr.org/2021/038.pdf).

## Getting Started

You will need to have Rust and Cargo installed.

```bash
# Build the project
$ cargo build --release

# Run all tests
$ cargo test
```

## Usage

### Run the project

```bash
$ cargo run --

# Sample response
Process started
Merkle root: [199, 41, 129, 186, 1, 206, 150, 229, 182, 26, 227, 214, 237, 148, 207, 70, 72, 179, 19, 153, 192, 148, 136, 224, 15, 89, 175, 72, 75, 143, 37, 140]
Time elapsed: 163.4682476s
```

### Code example

```Rust
use std::time::Instant;

use rust_merkle::root;

fn main() {
    let iterations = 11223344;
    let mut blockstream = Vec::with_capacity(iterations);

    for _i in 0..iterations {
        blockstream.push(Vec::from("12"))
    }

    let start = Instant::now();

    println!("Process started");

    let root = root(&blockstream);
    let elapsed = start.elapsed();

    println!("Merkle root: {:?}", root);
    println!("Time elapsed: {:?}", elapsed);
}
```

## To-do

Add the remaining proofs from the whitepaper:

- Single-leaf Proofs
- Single-range Proofs
- Multi-range Proofs

## References

- Streaming Merkle Proofs within Binary Numeral Trees, by [Luke Champine](https://eprint.iacr.org/2021/038.pdf)

## License

rust-merkle is distributed under the MIT license.
