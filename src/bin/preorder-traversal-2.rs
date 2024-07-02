enum Tree<T> {
    Leaf(T), Branch { value: T, left: Box<Tree<T>>, right: Box<Tree<T>>}
}

impl<'a, T: 'a> Tree<T> {
    fn iter(&self) -> impl Iterator<Item = &'a T> {
        let s = (*self);
        s.l
        [].iter();
    }
}

fn main() {
    
}
