extern crate std;
use std::vec::{Vec};

pub fn merge_sort<T: Ord + Clone>(q: &Vec<T>) -> Vec<T> {
    type Slice = (uint, uint);

    return merge_sort_(q, (0u, q.len()));

    fn merge_sort_<T: Ord + Clone>(v: &Vec<T>, slice: Slice) -> Vec<T> {
        let begin = slice.val0();
        let end   = slice.val1();

        let v_len = end - begin;
        if v_len == 0 {
            return vec!(v.get(begin).clone());
        }
        if v_len == 1 {
            return vec!(v.get(begin).clone());
        }

        let mid = begin + v_len / 2;
        let a = (begin, mid);
        let b = (mid, end);

        return merge(merge_sort_(v, a),
        merge_sort_(v, b));
    }

    fn merge<T: Ord + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        let (a_len, b_len) = (a.len(), b.len());
        let (mut a_ix, mut b_ix) = (0u, 0u);
        let mut rs = Vec::with_capacity(a_len + b_len);

        while a_ix < a_len && b_ix < b_len {
            if a.get(a_ix) < b.get(b_ix) {
                rs.push(a.get(a_ix).clone());
                a_ix += 1;
            } else {
                rs.push(b.get(b_ix).clone());
                b_ix += 1;
            }
        }

        rs.push_all(a.slice(a_ix, a_len));
        rs.push_all(b.slice(b_ix, b_len));
        rs
    }
}


pub fn insertion_sort<T: Ord + Clone>(q: &Vec<T>) -> Vec<T>{
    let mut xs = q.clone();
    for i in range(1, xs.len()) {
        let mut j = i;
        while j >= 1  && xs.get(j) < xs.get(j - 1){
            if xs.get(j) < xs.get(j - 1) {
                let temp = xs.get(j - 1).clone();
                *xs.get_mut(j - 1) = xs.get(j).clone();
                *xs.get_mut(j) = temp.clone();
                j -= 1;
            }
        }
    }
    return xs;
}
