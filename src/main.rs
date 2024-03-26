mod merkle_proof;
mod merkle_tree;
mod util;

use std::time::Instant;

use crate::merkle_tree::MerkleTree;

fn main() {
    let iterations = 4194304;
    let mut blockstream = Vec::with_capacity(iterations);

    for _i in 0..iterations {
        blockstream.push(Vec::from("12"))
    }

    let start = Instant::now();

    println!("Process started");

    let merkle_root = MerkleTree::root(&blockstream);
    let elapsed = start.elapsed();

    println!("Merkle root: {:?}", merkle_root);
    println!("Time elapsed: {:?}", elapsed);
}
