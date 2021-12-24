
pub fn print_vector() {
	println!("--------------- Vectors --------------------");
	let v: Vec<i32> = Vec::new();
	let v2 = vec![1,2,3];
	let mut v3 = Vec::new();

	v3.push(5);
	v3.push(6);

	{
		let v4 = vec![33];
		print!("{:?}", v4)
	}

	println!("ein vector {:?}", v);
	println!("ein vector {:?}", v2);
	println!("ein vector {:?}", v3);

	let third: &i32 = &v2[2];
	
	println!("value of third element: {:?}", third);
	match v2.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

	{
		let _v = vec![100, 32, 56];
		for i in &_v {
			println!("{}", i);
		}

		let mut _v2 = vec![100, 32, 57];
    for i in &mut _v2 {
        *i += 50;
    }
	}

	#[derive(Debug)]
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12)
	];

	println!("{:?}", row)
}