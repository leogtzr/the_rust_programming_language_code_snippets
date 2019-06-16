trait Flying {
    fn fly(&self);
}

#[derive(Debug)]
struct A {}

#[derive(Debug)]
struct B {}

impl Flying for A {
    fn fly(&self) {
        println!("In A");
    }
}

impl Flying for B {
    fn fly(&self) {
        println!("In B");
    }
}

fn something(f: impl Flying) {
    f.fly();
}

fn other<T: Flying>(item: T) {
    item.fly();
}

fn main() {
    something(A{});
    something(B{});
    other(A{});
}
