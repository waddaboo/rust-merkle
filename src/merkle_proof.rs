use std::collections::HashMap;

use crate::{
    merkle_tree::MerkleTree,
    util::{ones, zeros},
};

#[derive(Debug, Clone)]
pub struct Proof {
    pub pre: Vec<SubRoot>,
    pub leaf: Vec<u8>,
    pub post: Vec<SubRoot>,
}

#[derive(Debug, Clone)]
pub struct SubRoot {
    pub i: i32,
    pub subroot: Vec<u8>,
}

pub struct MerkleProof;

impl MerkleProof {
    pub fn read_leaf(stream: Vec<Vec<u8>>) -> Option<Vec<u8>> {
        if stream.is_empty() {
            return None;
        }

        Some(MerkleTree::leaf_hash(&stream[0]).to_vec())
    }

    pub fn prove_leaf(stream: Vec<Vec<u8>>, index: i32) -> Proof {
        let mut pre = Vec::with_capacity(ones(index).len());

        for &i in ones(index).iter() {
            pre.push(SubRoot {
                i,
                subroot: MerkleTree::subroot(&stream, i),
            })
        }

        let mut post = Vec::with_capacity(zeros(index).len());

        for &i in zeros(index).iter() {
            post.push(SubRoot {
                i,
                subroot: MerkleTree::subroot(&stream, i),
            })
        }

        let leaf = Self::read_leaf(stream).expect("Invalid leaf");

        Proof { pre, leaf, post }
    }

    pub fn load_stack(
        mut stack: HashMap<i32, Vec<u8>>,
        blocks: Vec<SubRoot>,
    ) -> HashMap<i32, Vec<u8>> {
        for block in blocks {
            stack = MerkleTree::insert(stack, &block.subroot, block.i)
        }

        stack
    }

    pub fn root_from_proof_and_leaf(leaf: Vec<u8>, proof: Proof) -> Vec<u8> {
        let mut stack = HashMap::new();
        stack = Self::load_stack(stack, proof.pre);
        stack = MerkleTree::insert(stack, &leaf, 0);
        stack = Self::load_stack(stack, proof.post);
        stack = MerkleTree::insert(stack, &leaf, 0);

        MerkleTree::finalize(stack)
    }

    pub fn verify_leaf(knownroot: Vec<u8>, leaf: Vec<u8>, proof: Proof) -> bool {
        knownroot == Self::root_from_proof_and_leaf(leaf, proof)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::merkle_proof::{Proof, SubRoot};

    use super::MerkleProof;

    #[test]
    fn test_read_leaf() {
        let stream = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let leaf = MerkleProof::read_leaf(stream);
        assert_eq!(
            leaf,
            Some(vec![
                3, 144, 88, 198, 242, 192, 203, 73, 44, 83, 59, 10, 77, 20, 239, 119, 204, 15, 120,
                171, 204, 206, 213, 40, 125, 132, 161, 162, 1, 28, 251, 129
            ])
        );
    }

    #[test]
    fn test_prove_leaf() {
        let stream = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let proof = MerkleProof::prove_leaf(stream, 1);
        assert_eq!(proof.pre.len(), 1);
        assert_eq!(proof.post.len(), 31);
        assert_eq!(
            proof.leaf,
            vec![
                3, 144, 88, 198, 242, 192, 203, 73, 44, 83, 59, 10, 77, 20, 239, 119, 204, 15, 120,
                171, 204, 206, 213, 40, 125, 132, 161, 162, 1, 28, 251, 129,
            ]
        );
    }

    #[test]
    fn test_load_stack() {
        let mut stack = HashMap::new();
        stack = MerkleProof::load_stack(
            stack,
            vec![SubRoot {
                i: 0,
                subroot: vec![1, 2, 3],
            }],
        );
        assert_eq!(stack.get(&0).unwrap(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_root_from_proof_and_leaf() {
        let leaf = vec![1, 2, 3];
        let proof = Proof {
            pre: vec![],
            leaf: leaf.clone(),
            post: vec![],
        };
        let root = MerkleProof::root_from_proof_and_leaf(leaf, proof);
        assert_eq!(
            root,
            vec![
                59, 163, 94, 232, 63, 33, 128, 19, 248, 184, 63, 8, 174, 44, 6, 201, 216, 224, 90,
                25, 141, 39, 28, 87, 201, 233, 161, 119, 27, 68, 162, 17
            ]
        );
    }

    #[test]
    fn test_verify_leaf() {
        let knownroot = vec![
            59, 163, 94, 232, 63, 33, 128, 19, 248, 184, 63, 8, 174, 44, 6, 201, 216, 224, 90, 25,
            141, 39, 28, 87, 201, 233, 161, 119, 27, 68, 162, 17,
        ];
        let leaf = vec![1, 2, 3];
        let proof = Proof {
            pre: vec![],
            leaf: leaf.clone(),
            post: vec![],
        };
        let verified = MerkleProof::verify_leaf(knownroot.clone(), leaf.clone(), proof);
        assert_eq!(verified, true);

        let nv_knownroot = vec![1, 2, 3];
        let nv_leaf = vec![1, 2, 3];
        let nv_proof = Proof {
            pre: vec![],
            leaf: leaf.clone(),
            post: vec![],
        };
        let not_verified =
            MerkleProof::verify_leaf(nv_knownroot.clone(), nv_leaf.clone(), nv_proof);
        assert_eq!(not_verified, false);
    }
}
