use std::process::{Command, Output};



fn main() {
    let Output {status, stdout, stderr } = Command::new("ls").output().expect("ls?");
    

    dbg!(String::from_utf8(stdout));

}
