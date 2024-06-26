use std::collections::HashMap;

use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

pub struct MerkleTree;

impl MerkleTree {
    pub fn leaf_hash(node: &[u8]) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(node);
        hasher.finalize().into()
    }

    pub fn parent_hash(left: &[u8], right: &[u8]) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().into()
    }

    pub fn fold_right<F>(f: F, col: &[&[u8]]) -> Vec<u8>
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

        Self::fold_right(Self::parent_hash, &values)
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

    pub fn limit(stream: &Vec<Vec<u8>>, limit: usize) -> Vec<Vec<u8>> {
        stream.into_iter().take(limit).cloned().collect()
    }

    pub fn subroot(stream: &Vec<Vec<u8>>, index: i32) -> Vec<u8> {
        let limit = Self::limit(stream, 1 << index);
        Self::root(&limit)
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
        let hash = MerkleTree::fold_right(
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

    #[test]
    fn test_limit() {
        let stream = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let limited_stream = MerkleTree::limit(&stream, 2);
        assert_eq!(limited_stream, vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let stream = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let limited_stream = MerkleTree::limit(&stream, 1);
        assert_eq!(limited_stream, vec![vec![1, 2, 3]]);

        let stream = vec![vec![1, 2, 3]];
        let limited_stream = MerkleTree::limit(&stream, 0);
        assert_eq!(limited_stream, Vec::<Vec<u8>>::new());

        let stream = vec![];
        let limited_stream = MerkleTree::limit(&stream, 1);
        assert_eq!(limited_stream, Vec::<Vec<u8>>::new());
    }

    #[test]
    fn test_subroot() {
        let data = vec![b"foo".to_vec()];
        let subroot = MerkleTree::subroot(&data, 0);
        assert_eq!(
            subroot,
            [
                44, 38, 180, 107, 104, 255, 198, 143, 249, 155, 69, 60, 29, 48, 65, 52, 19, 66, 45,
                112, 100, 131, 191, 160, 249, 138, 94, 136, 98, 102, 231, 174
            ]
        );

        let data = vec![b"foo".to_vec(), b"bar".to_vec(), b"doe".to_vec()];
        let subroot = MerkleTree::subroot(&data, 1);
        assert_eq!(
            subroot,
            [
                146, 71, 80, 4, 231, 15, 65, 185, 71, 80, 244, 167, 123, 247, 180, 48, 85, 17, 19,
                178, 93, 61, 87, 22, 158, 173, 202, 86, 146, 187, 4, 61
            ]
        );

        let data = vec![
            b"foo1".to_vec(),
            b"bar1".to_vec(),
            b"doe1".to_vec(),
            b"baz1".to_vec(),
            b"qux1".to_vec(),
        ];
        let subroot = MerkleTree::subroot(&data, 3);
        assert_eq!(
            subroot,
            [
                229, 152, 212, 157, 135, 146, 133, 149, 239, 201, 229, 11, 111, 55, 66, 235, 133,
                227, 254, 211, 196, 26, 38, 15, 143, 126, 64, 221, 225, 121, 22, 109
            ]
        );

        let data = vec![
            b"hello1".to_vec(),
            b"world1".to_vec(),
            b"merkle1".to_vec(),
            b"tree1".to_vec(),
            b"test1".to_vec(),
            b"example1".to_vec(),
        ];
        let subroot = MerkleTree::subroot(&data, 4);
        assert_eq!(
            subroot,
            [
                34, 121, 29, 7, 41, 224, 224, 178, 130, 193, 209, 50, 68, 26, 149, 252, 232, 98,
                114, 80, 229, 103, 119, 167, 73, 104, 15, 190, 223, 118, 5, 94
            ]
        );
    }
}
