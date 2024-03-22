use std::collections::HashMap;

use sha2::{Digest, Sha256};

fn leaf_hash(n: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(n);

    let result = hasher.finalize();
    let mut hash = [0; 32];
    hash.copy_from_slice(&result[..]);

    hash
}

fn parent_hash(left: &[u8], right: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);

    let result = hasher.finalize();
    let mut hash = [0; 32];
    hash.copy_from_slice(&result[..]);

    hash
}

fn foldr<F>(f: F, col: &[&[u8]]) -> Vec<u8>
where
    F: Fn(&[u8], &[u8]) -> [u8; 32],
{
    if col.is_empty() {
        return Vec::new();
    }

    let mut res = col[col.len() - 1].to_vec();

    for i in (0..col.len() - 1).rev() {
        let folded = f(col[i], &res);

        res = folded.to_vec();
    }

    res
}

fn insert(mut s: HashMap<i32, Vec<u8>>, v: &[u8], n: i32) -> HashMap<i32, Vec<u8>> {
    if let Some(val) = s.get(&n) {
        let p = parent_hash(val, v);
        s = delete(s, n);

        return insert(s, &p, n + 1);
    }

    s.insert(n, v.to_vec());

    s
}

fn delete(mut s: HashMap<i32, Vec<u8>>, n: i32) -> HashMap<i32, Vec<u8>> {
    s.remove(&n);

    s
}

fn finalize(s: HashMap<i32, Vec<u8>>) -> Vec<u8> {
    let mut keys: Vec<i32> = s.keys().cloned().collect();
    keys.sort_by(|a, b| b.cmp(a));

    let mut values = Vec::new();

    for k in keys {
        if let Some(v) = s.get(&k) {
            values.push(v.as_slice());
        }
    }

    foldr(parent_hash, &values)
}

pub fn root(stream: &Vec<Vec<u8>>) -> Vec<u8> {
    if stream.is_empty() {
        return Vec::new();
    }

    let mut m = HashMap::new();

    for v in stream.iter() {
        m = insert(m, &leaf_hash(&v), 0)
    }

    finalize(m)
}

#[cfg(test)]
mod tests {

    use crate::{foldr, leaf_hash, parent_hash, root};

    #[test]
    fn test_leaf_hash() {
        let hash = leaf_hash(b"Hello, World!");
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
        let hash = parent_hash(&[1, 2, 3], &[4, 5, 6]);
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
        let hashes: Vec<_> = data.iter().map(|&x| leaf_hash(x)).collect();
        let hash = foldr(
            parent_hash,
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
        let root = root(&flattened_data);
        assert_eq!(
            root,
            [
                106, 230, 42, 223, 172, 120, 111, 20, 132, 232, 170, 28, 105, 208, 226, 9, 220,
                242, 253, 221, 178, 5, 82, 130, 59, 32, 68, 5, 10, 45, 12, 146
            ]
        )
    }
}
