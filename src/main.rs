use colored::Colorize;
use dirs::home_dir;
use rand::{prelude::IteratorRandom, thread_rng};
use std::{
	fs::File,
	io::{stdin, stdout, BufRead, BufReader, Write},
	path::Path,
	process::Command,
};

fn main() {
	Command::new("clear")
		.status()
		.expect("Error clearing screen");

	println!(
		"\n  -----------------\n |   {} {} {} {} {} {}   |\n  -----------------\n",
		"R".bold().red(),
		"U".bold().yellow(),
		"S".bold().green(),
		"T".bold().cyan(),
		"L".bold().blue(),
		"E".bold().purple()
	);

	let target = BufReader::new(if Path::new("src/lib/wordlist.txt").exists() {
		File::open(Path::new("src/lib/wordlist.txt")).expect("Error resolving wordlist path")
	} else {
		File::open(Path::new(
			format!(
				"{}/bin/lib/wordlist.txt",
				home_dir()
					.expect("Error resolving home directory path")
					.to_str()
					.expect("")
			)
			.as_str(),
		))
		.expect("Error resolving wordlist path")
	})
	.lines()
	.map(|l| l.expect("Error reading wordlist"))
	.choose(&mut thread_rng())
	.expect("Error reading wordlist lines");

	let mut tries: u8 = 1;

	while tries <= 6 {
		print!("  {} -> ", tries);
		stdout().flush().expect("Error flushing stdout");

		let mut guess = String::new();
		stdin().read_line(&mut guess).unwrap();

		guess = guess.to_lowercase();

		if let Some('\n') = guess.chars().next_back() {
			guess.pop();
		}
		if let Some('\r') = guess.chars().next_back() {
			guess.pop();
		}

		if guess.chars().count() != target.chars().count() {
			println!(
				"  ! => The word is {} characters long\n",
				target.chars().count()
			);
			continue;
		}

		print!("  {} => ", if guess == target { "$" } else { " " });

		for (i, c) in guess.chars().enumerate() {
			if target.chars().nth(i) == Some(c) {
				print!("{}", c.to_string().bold().green());
				continue;
			}

			if target.find(c).is_some() {
				print!("{}", c.to_string().bold().yellow());
				continue;
			}

			print!("{}", c);
		}

		stdout().flush().expect("Error flushing stdout");

		println!();

		if guess == target {
			println!(
				"\n You guessed the word in {} tr{}!",
				tries.to_string().bold().blue(),
				if tries == 1 { "y" } else { "ies" }
			);
			break;
		}

		tries += 1;

		println!();
	}

	if tries > 6 {
		println!(" The word was {}", target.to_string().bold().blue());
	}
}
