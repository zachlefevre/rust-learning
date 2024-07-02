struct Mtx<T> {
    data: T
}

struct MtxGuard<'a, T> {
    data: &'a T
}

impl<T> Mtx<T> {
    fn lock(&self) -> MtxGuard<T> {
        MtxGuard {
            data: &self.data
        }
    }
}


fn main() {
    let data = "hey there".to_string();
    let m = Mtx { data };
    let v = m.lock();
}
