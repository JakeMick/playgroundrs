extern crate playgroundrs;
use playgroundrs::heap::Heap;
use playgroundrs::util::{disp_heap, rand, assert_sorted};

fn main() {
    let mut a = rand(10);
    println!("{}", a);
    a.heap_sort();
    let c = assert_sorted(&a);
    println!("{}", c);
    println!("{}", a);
}
