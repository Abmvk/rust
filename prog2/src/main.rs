use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn clear_screen() {
	let mut stdout = io::stdout();
	let _ = stdout.write(b"\x1B[2J\x1B[1;1H");
	stdout.flush().unwrap();
}

fn main() {

	clear_screen();
	let mut pogingen = 0;

	println!("Raad een getal");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {

		pogingen += 1;
	
		println!("Getal?");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Lezen niet gelukt");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Te laag"),
			Ordering::Greater => println!("Te hoog"),
			Ordering::Equal => {
				println!("Gewonnen in {pogingen} beurten");
				break;
			}
		}
	}

}

