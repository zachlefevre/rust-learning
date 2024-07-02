use serde::Deserialize;


#[derive(Deserialize)]
struct Foo {
    #[serde(rename="ayo")] a: i32
}


fn main() {
    
}
