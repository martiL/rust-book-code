use std::fs;
use std::fs::File;
use std::io::{self, Read, ErrorKind};

pub fn trigger_error() {
    let f = File::open("hello.txt");

    //  let f = match f {
    // 	 Ok(file) => file,
    // 	 Err(error) => panic!("problem opening the file {:?}", error)
    //  };

		
    let f = match f {
			Ok(file) => file,
			Err(error) => match error.kind() {
				ErrorKind::NotFound => match File::create("hello.txt") {
					Ok(fc) => fc,
					Err(e) => panic!("problem creating file {:?}", e),
				},
				other_error => panic!("problem opening the file {:?}", other_error),
			},
    };
		
		// easy way to get the file
		let file = File::open("hello.txt").unwrap();

		// easier to tag errors:
		let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
}

pub fn read_username_from_file() -> Result<String, io::Error> {
	// let f = File::open("hello.txt");

	// let mut f = match f {
	// 		Ok(file) => file,
	// 		Err(e) => return Err(e),
	// };

	// let mut s = String::new();

	// match f.read_to_string(&mut s) {
	// 		Ok(_) => Ok(s),
	// 		Err(e) => Err(e),
	// }

	// let mut f = File::open("hello.txt")?;
	// let mut s = String::new();
	// f.read_to_string(&mut s)?;
	// Ok(s)


	// short solution
	// let mut s = String::new();

	// File::open("hello.txt")?.read_to_string(&mut s)?;

	// Ok(s)

	// shortest solution
	fs::read_to_string("hello.txt")
}
