#[allow(dead_code)]
pub fn string_to_byte_slice(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

fn bit_test(i: i32, n: i32) -> bool {
    (i & (1 << n)) != 0
}

pub fn ones(i: i32) -> Vec<i32> {
    (0..32).filter(|&n| bit_test(i, n)).collect()
}

pub fn zeros(i: i32) -> Vec<i32> {
    (0..32).filter(|&n| !bit_test(i, n)).collect()
}
