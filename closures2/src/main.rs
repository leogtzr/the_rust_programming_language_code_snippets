use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

}

/*
Problem: it only accepts closures that take one parameter of type u32 and return a u32. 
We might want to cache 
the results of closures that take a string slice and return usize values, for example. 
To fix this issue, try introducing more generic parameters to increase the flexibility of 
the Cacher functionality.
*/

struct Cacher<T, A, B> where T: Fn(A) -> B {
    calculation: T,
    #[warn(dead_code)]
    map: HashMap<A, B>,
}

impl<T, A: Hash + Eq + Copy, B: Copy> Cacher<T, A, B> where T: Fn(A) -> B {
    fn new(calculation: T) -> Cacher<T, A, B> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> B {
        match self.map.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

}