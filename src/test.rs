use heap::{Heap};
use sort::{insertion_sort, merge_sort};
use util::{assert_sorted, rand};

#[test]
fn test_min_heap_insert() {
    let mut a = rand(10001);
    a.build_min_heap();
    a.min_heap_insert(10);
    assert_min_heap(&a);
}

#[test]
fn test_heap_min() {
    let a = vec!(0,1);
    let b = a.heap_min();
    assert!(b == &0);
}

#[test]
fn test_heap_swap() {
    let mut h = Vec::from_fn(3, |i| i);
    h.swap(1, 2);
    h.swap(0, 1);
    h.swap(2, 1);
    let g = vec!(2, 1, 0);
    assert!(h == g);
}

#[test]
fn test_heap_extract_min() {
    let mut a = rand(100);
    a.build_min_heap();
    let mut b = a.heap_extract_min();
    let mut c = a.heap_extract_min();
    for i in range(0, a.len() / 2 - 1) {
        if c < b {
            assert!(false);
        }
        b = c;
        c = a.heap_extract_min();
        assert_min_heap(&a);
    }
}

#[test]
fn test_oppo_heap_sort() {
    let mut a = rand(100000);
    a.heap_sort();
    assert!(a.as_slice().windows(2).all(|pair| pair[1] <= pair[0]));
}

fn assert_min_heap(xs: &Vec<int>) {
    let xs_len = xs.len();
    for (ind, val) in xs.iter().enumerate() {
        let l = xs.left(ind);
        let r = xs.right(ind);
        if l < xs_len {
            if xs.get(l) < val {
                assert!(false);
            }
        }
        if r < xs_len {
            if xs.get(r) < val {
                assert!(false);
            }
        }
    }
}


#[test]
fn test_build_min_heap() {
    let mut a = rand(100);
    a.build_min_heap();
    assert_min_heap(&a);
}

#[test]
fn test_merge(){
    let a = rand(100);
    let b = merge_sort(&a);
    let c = assert_sorted(&b);
    assert!(c);
}

#[test]
fn test_insertion(){
    let a = rand(100);
    let b = insertion_sort(&a);
    let c = assert_sorted(&b);
    assert!(c);
}


