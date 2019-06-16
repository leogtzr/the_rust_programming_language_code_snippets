trait A {
    fn forA(&self);
}

trait B {
    fn forB(&self);
}

struct TheA {}

impl A for TheA {
    fn forA(&self) {
        println!("A in TheA");
    }
}

impl B for TheA {
    fn forB(&self) {
        println!("B in TheB");
    }
}

fn something<T: A + B>(item: T) {
    item.forA();
    item.forB();
}

fn something2(item: impl A + B) {
    item.forA();
    item.forB();
}

fn something3<T>(item: T) where T: A {
    item.forA();
}

fn something4<T>(item: T) where T: A + B {
    item.forA();
    item.forB();
}

// returning a type that implements A
fn newA() -> impl A {
    TheA{}
}

fn main() {
    something(TheA{});
    something2(TheA{});
    something3(TheA{});
    something4(TheA{});
}
