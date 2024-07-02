//     You start with 1.
//     You can mutiply your number N by 2 times. You can add as many time as you want for free.
//     How many operations does it take to get to some target T

fn solve(target: i32, mults: usize) -> usize {
    println!("{} -- {}", target, mults);
    if target == 1 {
        0
    } else if target % 2 == 1 {
        1 + solve(target - 1, mults)
    } else if mults > 0 {
        1 + solve(target / 2, mults - 1)
    } else {
        target as usize
    }
}

fn main() {
    dbg!(solve(22, 2));
    println!("Hello, world!");
}
