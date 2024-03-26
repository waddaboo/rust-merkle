use rust_merkle::MerkleTree;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let elapsed = start.elapsed();

    // Create some data to insert into the Merkle tree
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];

    let flattened_data = data.into_iter().collect();

    // Calculate the root hash of the tree
    let root = MerkleTree::root(&flattened_data);
    println!("Root hash: {:?}", root);

    // Compute a subroot hash at a given index
    let subroot = MerkleTree::subroot(&flattened_data, 0);
    println!("Subroot hash: {:?}", subroot);

    let subroot2 = MerkleTree::subroot(&flattened_data, 2);
    println!("Subroot hash at index 2: {:?}", subroot2);

    // Limit the tree to a certain number of nodes
    let limited_tree = MerkleTree::limit(&flattened_data, 3);
    let limited_root = MerkleTree::root(&limited_tree);
    println!("Limited root hash: {:?}", limited_root);

    println!("Time elapsed: {:?}", elapsed);
}
