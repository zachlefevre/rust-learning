use std::collections::BTreeMap;

fn main() {
    let mut btreemap = BTreeMap::new();
    let elements = (0..100);
    for element in elements {
        *btreemap.entry(element / 2).or_insert(0) += 1
    }
    dbg!(btreemap);
}
