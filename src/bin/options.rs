

fn main() {
    let op = Some(2);
    let value = op.unwrap_or(18);
    dbg!(op);

    let op = Some(String::from("hey there"));
    let value = op.unwrap_or("hey there".to_string());
    //    dbg!(op); -- not allowed because String: !Copy
}

