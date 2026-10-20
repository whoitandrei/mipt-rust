#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }
    if arr.len() < k {
        return vec![];
    }

    let mut res = combinations(&arr[1..], k - 1);
    for data in res.iter_mut() {
        data.insert(0, arr[0]);
    }

    res.extend(combinations(&arr[1..], k));
    res
}
