enum dothingErrors {
    SomethingWentWrong(String)
}

fn dothing(s: String) -> Result<(), dothingErrors> {
    println!("trying to dothing");
    Err(dothingErrors::SomethingWentWrong(s))
}

fn retry<T, E, R>(t: T, f: fn(T) -> Result<R, E>, extractor: fn(E) -> T, times: i32) -> Result<R, E> {
    match  f(t) {
        Err(e) if times >= 0 =>                retry(extractor(e), f, extractor, times - 1),
        Err(e) => Err(e),
        Ok(v) => Ok(v)
    }
}

fn main() {
    let s = String::from("hey there person");
    retry(s, dothing, |e| {
        match e {
            dothingErrors::SomethingWentWrong(e) => e
        }
    }, 1);
}
