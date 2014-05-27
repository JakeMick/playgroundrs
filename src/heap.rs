// heap.rs

//! Utility functions and algorithms for minimun heaps.


/// I implemented this as a trait.
///
/// If I was only interested in heap_sort, I'd write something like
/// fn heap_sort<T: Ord>(&mut Vec<T>) {
///      fn min_heapify<T: Ord>(&mut Vec<T>) { ...
///      }
///      ...
/// }

use std::ptr::{swap};
use std::vec::{Vec};


pub trait Heap<T> {
    fn heap_sort(&mut self);
    fn heap_min<'a>(&'a self) -> &'a T;
    fn heap_extract_min(&mut self) -> T;
    fn min_heapify(&mut self, uint, uint);
    fn build_min_heap(&mut self);
    fn left(&self, uint) -> uint;
    fn right(&self, uint) -> uint;
    fn parent(&self, uint) -> uint;
    fn swap(&mut self, uint, uint);
}

impl <T: Ord> Heap<T> for Vec<T> {
    fn heap_sort(&mut self) {
        self.build_min_heap();
        for i in range(1, self.len()).rev()  {
            self.swap(0, i);
            self.min_heapify(0, i);
        }
    }

    fn heap_min<'a>(&'a self) -> &'a T {
        self.get(0)
    }

    fn heap_extract_min(&mut self) -> T {
        let o = self.swap_remove(0);
        let self_len = self.len();
        self.min_heapify(0, self_len);
        return o.unwrap()
    }

    fn min_heapify(&mut self, i: uint, j: uint) {
        let l = self.left(i);
        let r = self.right(i);
        let mut smallest = i;
        if l < j && self.get(l) > self.get(smallest) {
            smallest = l;
        } 
        if r < j && self.get(r) > self.get(smallest) {
            smallest = r;
        }
        if smallest != i {
            self.swap(i, smallest);
            self.min_heapify(smallest, j);
        }
    }

    fn build_min_heap(&mut self) {
        let self_len = self.len();
        for i in range(0, self_len / 2 + 1).rev() {
            self.min_heapify(i, self_len);
        }
    }

    fn left(&self, i: uint) -> uint {
        2 * i + 1
    }

    fn right(&self, i: uint) -> uint {
        2 * i + 2
    }

    fn parent(&self, i: uint) -> uint {
        i / 2
    }

    fn swap(&mut self, i: uint, j: uint) {
        unsafe{
            let ptr_to_i: *mut T = self.get_mut(i);
            let ptr_to_j: *mut T = self.get_mut(j);
            swap(ptr_to_i, ptr_to_j);
        }
    }

}
