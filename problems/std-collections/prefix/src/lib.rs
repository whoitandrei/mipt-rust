#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut iters: Vec<_> = strs.iter().map(|s| s.chars()).collect();
    let mut res = String::new();

    loop {
        let Some(c) = iters[0].next() else { return res };
        for it in iters.iter_mut().skip(1) {
            if it.next() != Some(c) {
                return res;
            }
        }
        res.push(c);
    }
}
