trait Show {
    fn show_ref(&self);
    fn show(self);
}

struct Person(String);
impl Show for Person {
    fn show_ref(&self) {
        println!("showing {}", self.0)
    }
    fn show(self) {
        println!("showing {}", self.0)
    }
}

impl Show for i32 {
    fn show_ref(&self) {
        println!("showing number {}", self)
    }
    fn show(self) {
        println!("showing number {}", self)
    }
}

fn main() {
    let items: Vec<Box<dyn Show>> = vec![Box::new(88), Box::new(Person("YO".into()))];
    for item in items {
        item.show_ref()
    };

    let p = Person("YO".into());
    let items: Vec<&dyn Show> = vec![&88, &p];
    for item in items {
        item.show_ref()
    }


}
