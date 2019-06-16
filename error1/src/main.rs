use std::fs::File;

fn main() {
    // panic!("crash and burn");
    
    //let v = vec![1, 2, 3, 4, 5];
    //v[99];
    
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err)
        },
    }; 
    
}
