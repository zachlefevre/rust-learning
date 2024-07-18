trait Animal {
    fn sound(&self) -> String;
}

fn dynamic_dispatch(_a: &dyn Animal) {
    
}

fn static_dispatch_reference(_a: &impl Animal) {
    
}

fn static_dispatch_own(_a: impl Animal) {
    
}


struct Wombat {
}

impl Animal for Wombat {
    fn sound(&self) -> String {
        "womba noises".into()
    }
}

fn main() {
    let w = Wombat {};
    dynamic_dispatch(&w);
    static_dispatch_reference(&w);
    static_dispatch_own(w);
}
