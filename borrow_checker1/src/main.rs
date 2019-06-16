// Lifetime annotations describe the relationships of the lifetimes of multiple 
// references to each other without affecting the lifetimes.

/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

/* It won't compile: */

fn lonest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}
