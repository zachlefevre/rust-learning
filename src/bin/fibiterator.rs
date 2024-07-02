impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut curr = 0;
        let mut next = 1;
        for _ in 0..n {
            let prev = next;
            next = next + curr;
            curr = prev;
        };
        curr
    }
}

struct Solution;

fn main() {
    dbg!(Solution::fib(5));
}
