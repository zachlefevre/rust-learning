use Tree::*;

#[derive(Debug)]
enum Tree<T> {
    Leaf(T), Branch{ value: T, left: Box<Tree<T>>, right: Box<Tree<T>> }
}

#[derive(Debug)]
struct PreorderTraversal<'a, T> {
    stack: Vec<&'a Tree<T>>
}

impl<'a, T> PreorderTraversal<'a, T> {
    fn new(tree: &'a Tree<T>) -> Self { Self { stack: vec![tree] } }
}

impl<'a, T> Iterator for PreorderTraversal<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(tree) => match tree {
                Tree::Leaf(v) => Some(v),
                Tree::Branch { value, left, right } => {
                    self.stack.push(right);
                    self.stack.push(left);
                    Some(value)
                }
            },
            None => None
        }
    }
}

fn main() {
    let tree = &Branch{value: 0, left: Box::new(Leaf(1)), right: Box::new(Leaf(2))};
    dbg!(PreorderTraversal::new(tree).collect::<Vec<_>>());
}
