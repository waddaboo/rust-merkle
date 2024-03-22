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

    let merkle_root = root(&blockstream);
    let elapsed = start.elapsed();

    println!("Merkle root: {:?}", merkle_root);
    println!("Time elapsed: {:?}", elapsed);
}
