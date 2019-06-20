use std::thread;
use std::time::Duration;

use std::collections::HashMap;

/*

Closure syntax:

fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
*/


// #[warn(dead_code)]
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly ...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_result = Cacher::new(|num| {
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
    // println!("Hello, world!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
    #[warn(dead_code)]
    map: HashMap<u32, u32>,
}

/*
Try modifying Cacher to hold a hash map rather than a single value. The keys of 
the hash map will be the arg values that are passed in, and the values of the hash 
map will be the result of calling the closure on that key. Instead of looking at 
whether self.value directly has a Some or a None value, the value function will look 
up the arg in the hash map and return the value if it’s present. If it’s not present, 
the Cacher will call the closure and save the resulting value in the hash map associated 
with its arg value.
*/

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // match self.value {
        //     Some(v) => v,
        //     None =>  {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     },
        // }

        match self.map.get(&arg) {
            Some(v) => *v,
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