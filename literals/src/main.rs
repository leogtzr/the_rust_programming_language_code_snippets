fn main() {
    let a: u32 = 98_222;
    let b: u32 = 0xff;
    let c: u32 = 0o77;
    let d: u32 = 0b111_00;
    let e: u8 = b'A';
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);

    let aa: char = 'a';
    println!("{}", aa);

    let bb: (u32, char, bool) = (3, '_', true);
    println!("{}", bb.0);
    println!("{}", bb.1);
    println!("{}", bb.2);

    // Using types:
   	let (t1, t2, t3): (u32, char, bool) = bb;
   	println!("{}", t1);
   	println!("{}", t2);
   	println!("{}", t3);

   	println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>");
   	let ar = [1, 2, 3];
   	println!("{}", ar[0]);
   	println!("{}", ar.len());

   	for x in 0..ar.len() {
   		println!("{}", x);
   	}

   	let br: [i32; 3] = [56, 34, 87];
   	println!("{}", br[0]);
   	println!("{}", br[1]);
   	println!("{}", br[2]);

   	let mut cr: [i32; 2];
   	cr = [1, 3];

   	cr[0] = 34;
}