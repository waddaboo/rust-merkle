use std::collections::HashMap;

use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct MerkleTree;

impl MerkleTree {
    pub fn leaf_hash(node: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(node);

        let result = hasher.finalize();
        let mut hash = [0; 32];
        hash.copy_from_slice(&result[..]);

        hash
    }

    pub fn parent_hash(left: &[u8], right: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);

        let result = hasher.finalize();
        let mut hash = [0; 32];
        hash.copy_from_slice(&result[..]);

        hash
    }

    pub fn foldr<F>(f: F, col: &[&[u8]]) -> Vec<u8>
    where
        F: Fn(&[u8], &[u8]) -> [u8; 32],
    {
        if col.is_empty() {
            return Vec::new();
        }

        let mut result = col[col.len() - 1].to_vec();

        for i in (0..col.len() - 1).rev() {
            let folded = f(col[i], &result);

            result = folded.to_vec();
        }

        result
    }

    pub fn insert(
        mut stack: HashMap<i32, Vec<u8>>,
        value: &[u8],
        node: i32,
    ) -> HashMap<i32, Vec<u8>> {
        if let Some(val) = stack.get(&node) {
            let parent = Self::parent_hash(val, value);
            stack = Self::delete(stack, node);

            return Self::insert(stack, &parent, node + 1);
        }

        stack.insert(node, value.to_vec());

        stack
    }

    fn delete(mut stack: HashMap<i32, Vec<u8>>, node: i32) -> HashMap<i32, Vec<u8>> {
        stack.remove(&node);

        stack
    }

    pub fn finalize(stack: HashMap<i32, Vec<u8>>) -> Vec<u8> {
        let mut keys: Vec<i32> = stack.keys().cloned().collect();
        keys.sort_by(|a, b| b.cmp(a));

        let mut values = Vec::new();

        for key in keys {
            if let Some(value) = stack.get(&key) {
                values.push(value.as_slice());
            }
        }

        Self::foldr(Self::parent_hash, &values)
    }

    pub fn root(stream: &Vec<Vec<u8>>) -> Vec<u8> {
        if stream.is_empty() {
            return Vec::new();
        }

        let mut map = HashMap::new();

        for value in stream.iter() {
            map = Self::insert(map, &Self::leaf_hash(&value), 0)
        }

        Self::finalize(map)
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTree;

    #[test]
    fn test_leaf_hash() {
        let hash = MerkleTree::leaf_hash(b"Hello, World!");
        assert_eq!(
            hash,
            [
                223, 253, 96, 33, 187, 43, 213, 176, 175, 103, 98, 144, 128, 158, 195, 165, 49,
                145, 221, 129, 199, 247, 10, 75, 40, 104, 138, 54, 33, 130, 152, 111
            ]
        );
    }

    #[test]
    fn test_parent_hash() {
        let hash = MerkleTree::parent_hash(&[1, 2, 3], &[4, 5, 6]);
        assert_eq!(
            hash,
            [
                113, 146, 56, 92, 60, 6, 5, 222, 85, 187, 148, 118, 206, 29, 144, 116, 129, 144,
                236, 179, 42, 142, 237, 127, 82, 7, 179, 12, 246, 161, 254, 137
            ]
        );
    }

    #[test]
    fn test_foldr() {
        let data = vec![b"foo", b"bar", b"doe"];
        let hashes: Vec<_> = data.iter().map(|&x| MerkleTree::leaf_hash(x)).collect();
        let hash = MerkleTree::foldr(
            MerkleTree::parent_hash,
            hashes
                .iter()
                .map(|x| x.as_ref())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        assert_eq!(
            hash,
            [
                80, 153, 174, 172, 164, 113, 228, 182, 74, 214, 224, 141, 69, 149, 231, 210, 175,
                77, 21, 101, 63, 152, 2, 244, 233, 18, 230, 185, 49, 41, 217, 94
            ]
        );
    }

    #[test]
    fn test_root() {
        let data = vec![
            vec![b"foo".to_vec()],
            vec![b"bar".to_vec()],
            vec![b"doe".to_vec()],
        ];
        let flattened_data = data.into_iter().flatten().collect();
        let root = MerkleTree::root(&flattened_data);
        assert_eq!(
            root,
            [
                106, 230, 42, 223, 172, 120, 111, 20, 132, 232, 170, 28, 105, 208, 226, 9, 220,
                242, 253, 221, 178, 5, 82, 130, 59, 32, 68, 5, 10, 45, 12, 146
            ]
        )
    }
}
