use heap::Heap;

pub fn assert_sorted<T: Ord>(xs: &Vec<T>) -> bool {
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

pub fn disp_heap(xs: &Vec<int>) {
    let xs_len = xs.len();
    println!("Length: {}", xs_len);
    for (ind, val) in xs.iter().enumerate() {
        println!("Parent ind: {}, val: {}", ind, val);
        let l = xs.left(ind);
        let r = xs.right(ind);
        if l < xs_len {
            println!("  Left ind: {}, val: {}", l, xs.get(l));
        }
        if r < xs_len {
            println!("  Right ind: {}, val: {}", r, xs.get(r));
        }
    }
}
