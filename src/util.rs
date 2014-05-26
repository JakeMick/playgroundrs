pub fn assert_sorted<T: Ord>(xs: Vec<T>) -> bool {
    xs.as_slice().windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn rand(n: uint) -> Vec<int>{
    use std::vec::{Vec};
    use rand::{task_rng, Rng};

    let mut rng = task_rng();
    let mut x = Vec::with_capacity(n);
    for _ in range(0, n) {
        x.push(rng.gen_range(0, n as int))
    }
    return x;
}
