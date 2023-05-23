use std::io;

fn main()
{
	println!("Wat is je naam? ");
	let mut naam = String::new();
	io::stdin().read_line(&mut naam).expect("Kon de invoer niet lezen");
	let naam = naam.trim();

	println!("Hoe oud ben je? ");
	let mut leeftijd = String::new();
	io::stdin().read_line(&mut leeftijd).expect("Kon de invoer niet lezen");
	let leeftijd: u32 = leeftijd.trim().parse().expect("Ongeldige leeftijd");

	let begroeting = if leeftijd < 18
		{ 
		format!("Hallo {}, hoe gaat het met je?", naam)
		}
		else
		{
		format!("Hallo {}, hoe gaat het met u?", naam)
		};

	println!("{}", begroeting);
}

