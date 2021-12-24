pub fn print_string(){
	println!("--------------- Strings --------------------");
	//let mut s = String::new();

	let data = "inital contents";

	let s = data.to_string();
	let s = "initial contents".to_string();
	let mut s = String::from("initial contents");
	println!("string s is: {}", s);

	let hello = String::from("السلام عليكم");
	let hello = String::from("Dobrý den");
	let hello = String::from("Hello");
	let hello = String::from("שָׁלוֹם");
	let hello = String::from("नमस्ते");
	let hello = String::from("こんにちは");
	let hello = String::from("안녕하세요");
	let hello = String::from("你好");
	let hello = String::from("Olá");
	let hello = String::from("Здравствуйте");
	let hello = String::from("Hola");

	s.push_str("string");
	println!("string s is: {}", s);

	s.push('l');
	println!("string s is: {}", s);

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
  println!("string s3 is: {}", s3);

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = format!("{}-{}-{}", s1, s2, s3);
	println!("string s is: {}", s);

	let hello = "Здравствуйте";
	let s = &hello[0..4];
	println!("s is {}", s);

	for c in "नमस्ते".chars() {
    println!("{}", c);
	}

	for b in "नमस्ते".bytes() {
    println!("{}", b);
	}

	let ch = s.chars().nth(0).unwrap();
	println!("ch is {}", ch);
}
