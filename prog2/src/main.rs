use std::io;

fn main() {
	println!("Raad een getal");
	println!("Getal?");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Lezen niet gelukt");

	println!("Je getal is {guess}");


}

