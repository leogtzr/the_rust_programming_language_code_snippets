/*
Elision rules:
The patterns programmed into Rust’s analysis of references are called the lifetime 
elision rules. These aren’t rules for programmers to follow; they’re a set of 
particular cases that the compiler will consider, and if your code fits these cases,
 you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference. If Rust deterministically applies 
the rules but there is still ambiguity as to what lifetimes the references have, 
the compiler won’t guess what the lifetime of the remaining references should be. 
In this case, instead of guessing, the compiler will give you an error that you can 
resolve by adding the lifetime annotations that specify how the references relate 
to each other.

Lifetimes on function or method parameters are called input lifetimes, 
and lifetimes on return values are called output lifetimes.
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    
}
