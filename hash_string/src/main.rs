extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut sh = Md5::new();
    sh.input_str("hola");
    println!("{}", sh.result_str());
}