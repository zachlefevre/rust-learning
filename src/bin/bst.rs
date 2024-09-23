#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: std::cmp::PartialOrd> Node<T> {
    fn new(t: T) -> Self {
        Self {payload: t, left: None, right: None}
    }

    fn set_left(&mut self, left: T) {
        self.left = Some(Box::new(Node::new(left)))
    }
    fn set_right(&mut self, right: T) {
        self.right = Some(Box::new(Node::new(right)))
    }


    fn insert(&mut self, value: T) {
        if value < self.payload {
            match self.left {
                None => {
                    self.set_left(value)
                },
                Some(ref mut left) => left.insert(value)
            }
        }
        else {
            match self.right {
                None => {
                    self.set_right(value)
                },
                Some(ref mut right) => right.insert(value)
            }
        }
    }

}


fn main() {
    let mut n = Node::new(18);

    n.set_left(5);

//    n.left.as_mut().map(|left| left.add_left(-1));

    n.insert(-1);
    n.insert(-8);
    n.insert(8);

    dbg!(n);
}
