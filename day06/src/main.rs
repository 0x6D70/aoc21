use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").unwrap();

	let mut lines: Vec<String> = Vec::new();

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();
	
		lines.push(line);	
	}

	let mut numbers : Vec<i8> = lines[0].split(',').map(|x| x.parse::<i8>().unwrap()).collect();
	let orig = numbers.clone();

	for _i in 0..80 {
		let mut counter = 0;
		let length = numbers.len();

		for number in &mut numbers {
			*number -= 1;
		}

		while counter < length {
			if numbers[counter] == -1 {
				numbers[counter] = 6;
				numbers.push(8);	
			}

			counter += 1;
		}
	}

	println!("result = {}", numbers.len());

	// bad calculation, but it works
	// it would be better to save the already known results

	let mut result2 = 0;
	let mut i = 0;

	while i < orig.len() {
		let mut fish : Vec<i8> = vec![orig[i]];

		for _i in 0..256 {
			let mut counter = 0;
			let length = fish.len();

			for number in &mut fish {
				*number -= 1;
			}

			while counter < length {
				if fish[counter] == -1 {
					fish[counter] = 6;
					fish.push(8);
				}

				counter += 1;
			}

			let progress = (i as f64 + (_i as f64 / 256.0)) / orig.len() as f64 * 100.0;
			println!("i = {} orig.len() = {} _i = {} progress = {}", i, orig.len(), _i, progress);
		}

		result2 += fish.len();

		println!("result2 = {}", result2);	

		i += 1;
	}

	println!("result2 = {}", result2);
}
