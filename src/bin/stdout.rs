use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    let v = vec![97, 98, 99];
    stdout.write_all(&v[..]);

    let v2 = b"\nthis is some more stuff\n";
    stdout.write_all(v2);

    let v3 = vec![b'a', b'b', b'c'];
    stdout.write_all(&v3[..]);


    write!(stdout, "write! writes a string literal\n");
    let s = "it can also write other stuff with a template string".to_string();
    write!(stdout, "{}", s);
}
