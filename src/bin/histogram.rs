use std::collections::BTreeMap;

fn main() {
    let mut hist = BTreeMap::new();

    for i in 1..100 {
        let entry = hist.entry(i).or_insert(i);
        *entry += 1;
    }

    dbg!(hist);
}
