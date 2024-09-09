use std::{cmp::Reverse, collections::BinaryHeap};

fn smallest(items: impl Iterator<Item = i32>, k: u32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for item in items {
        if heap.len() >= k as usize {
            let largest: i32 = heap.pop().unwrap();
            let largest = largest.min(item);
            heap.push(largest)
        } else {
            heap.push(item)
        }
    }

    heap.into_vec()
}

fn largest(items: impl Iterator<Item = i32>, k: u32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for item in items {
        if heap.len() >= k as usize {
            let Reverse(largest): Reverse<i32> = heap.pop().unwrap();
            let largest = Reverse(largest.max(item));
            heap.push(largest)
        } else {
            heap.push(Reverse(item))
        }
    }

    heap.into_iter().map(|Reverse(n)| n).collect()
}


fn main() {
    dbg!(largest([6, 1, 2, 3, 4, 5, 4, 3, 2, 1, 8].into_iter(), 4));
}
