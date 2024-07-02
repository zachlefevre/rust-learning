struct Person {
    name: String
}

impl std::ops::Index<f32> for Person {
    type Output = String;

    fn index(&self, i: f32) -> &Self:: Output {
        &self.name
    }
}

fn main() {
    let p = Person {name: "Zachary".to_string() };
    dbg!(&p[8.8]);
}
