trait Show {}

impl Show for i32 {
    
}

fn main() {
    let t: i32 = 88;
    let items: Vec<&dyn Show> = vec![&1];
}
