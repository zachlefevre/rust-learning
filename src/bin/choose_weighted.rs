use rand::seq::SliceRandom;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1];
   dbg!(v.choose_weighted(&mut rand::thread_rng(), |x| *x));
}
