use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("Sarching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

/*
    The Trade-Offs of Using clone

    There’s a tendency among many Rustaceans to avoid using clone to fix ownership 
    problems because of its runtime cost. In Chapter 13, you’ll learn how to use 
    more efficient methods in this type of situation. But for now, it’s okay to
    copy a few strings to continue making progress because you’ll make these copies 
    only once and your filename and query string are very small. It’s better to have a working program 
    that’s a bit inefficient than to try to hyperoptimize code on your first pass. 
    As you become more experienced with Rust, it’ll be easier to start with the most 
    efficient solution, but for now, it’s perfectly acceptable to call clone.

*/
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config {
//         query, filename
//     }
// }