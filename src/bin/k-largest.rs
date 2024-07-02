use std::{collections::BinaryHeap, cmp::Reverse};

fn solve(items: &[i32], k: usize) -> Vec<i32>{
    let mut heap = BinaryHeap::with_capacity(items.len());
    for &item in items {
        if heap.len() < k {
            heap.push(Reverse(item))
        } else {
            let Reverse(smallest) = heap.pop().unwrap();
            heap.push(Reverse(smallest.max(item)))
        }
    }
    let removed = heap.iter().map(|&Reverse(item)| item);
    removed.collect::<Vec<_>>()
}


fn main() {
    dbg!(solve(&[1], 1
));
}
