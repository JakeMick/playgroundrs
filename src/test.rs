use heap::{Heap};
use sort::{insertion_sort, merge_sort};
use util::{assert_sorted, rand};

#[test]
fn test_min_heap_insert() {
    let mut a = rand(100);
    a.build_min_heap();
    a.min_heap_insert(10);
    assert_min_heap(a);

}

#[test]
fn test_heap_min() {
    let a = vec!(0,1);
    let b = a.heap_min();
    assert!(b == &0);
}

#[test]
fn test_heap_sort() {
    let mut a = rand(100);
    a.heap_sort();
    let c = assert_sorted(a);
    assert!(c);
}

#[test]
fn test_heap_swap() {
    let mut h = Vec::from_fn(3, |i| i);
    h.swap(1, 2);
    let g = vec!(0, 2, 1);
    assert!(h == g);
}

#[test]
fn test_heap_extract_min() {
    let mut a = rand(100);
    a.build_min_heap();
    let mut b = a.heap_extract_min();
    let mut c = a.heap_extract_min();
    for i in range(0, a.len() / 2 - 1) {
        if c > b {
            assert!(false);
        }
        b = c;
        c = a.heap_extract_min();
    }
}

fn assert_min_heap(a: Vec<int>) {
    for i in range(0, a.len() / 2 - 1) {
        let c = a.get(i);
        if a.get(a.left(i)) > c || a.get(a.right(i)) > c {
            assert!(false);
        }
    }

}

#[test]
fn test_build_min_heap() {
    let mut a = rand(100);
    a.build_min_heap();
    assert_min_heap(a);
}

#[test]
fn test_merge(){
    let a = rand(100);
    let b = merge_sort(&a);
    let c = assert_sorted(b);
    assert!(c);
}

#[test]
fn test_insertion(){
    let a = rand(100);
    let b = insertion_sort(&a);
    let c = assert_sorted(b);
    assert!(c);
}


