enum SpreadSheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

fn main() {
    let row = vec![
    	SpreadSheetCell::Int(3),
    	SpreadSheetCell::Text(String::from("blue")),
    	SpreadSheetCell::Float(10.12)
    ];

    match row.get(1) {
    	None => {
    		println!("Empty ... :(");
    	},
    	Some(val) => {
    		//println!("{:?}", val);
    		match val {
    			SpreadSheetCell::Int(x) => {
    				println!("int val: {}", x);
    			},
    			SpreadSheetCell::Text(text) => {
    				println!("text val: {}", text);
    			},
    			SpreadSheetCell::Float(x) => {
    				println!("float val: {}", x);
    			}
    		}
    	}
    }
}
