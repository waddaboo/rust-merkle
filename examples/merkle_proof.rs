use std::time::Instant;

use rust_merkle::MerkleProof;

fn main() {
    let start = Instant::now();
    let elapsed = start.elapsed();

    // Create a stream of hash values
    let stream = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // Prove the existence of a leaf node at index 1
    let proof = MerkleProof::prove_leaf(stream, 1);

    // Assume that we have a known root value from the Merkle tree
    let knownroot = vec![
        148, 116, 171, 150, 234, 181, 6, 118, 128, 13, 166, 250, 147, 231, 112, 159, 151, 214, 9,
        224, 6, 54, 211, 103, 161, 31, 161, 121, 251, 31, 152, 244,
    ];

    // Verify that the leaf node exists in the Merkle tree
    let verified = MerkleProof::verify_leaf(knownroot.clone(), proof.leaf.clone(), proof.clone());

    // Print the result of the verification
    println!("Leaf exists in Merkle tree: {}", verified);

    // You can also obtain the root of the Merkle tree from the proof and leaf
    let root = MerkleProof::root_from_proof_and_leaf(proof.leaf.clone(), proof);

    // Compare the computed root with the known root for validation
    println!("Computed root: {:?}", root);
    println!("Known root: {:?}", knownroot);
    println!("Roots match: {}", root == knownroot);

    println!("Time elapsed: {:?}", elapsed);
}
