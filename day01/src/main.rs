use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let mut values = Vec::<i32>::new();

	let file = File::open("input.txt").unwrap();

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();
		let number = line.parse::<i32>().unwrap();
		values.push(number);
	}

	let mut counter = 0;
	let mut i = 0;
	
	while i < values.len() {
		if i != 0 && values[i] > values[i-1] {
			counter += 1;
		}

		i += 1;
	}

	println!("count = {}", counter);

	counter = 0;
	let mut i = 3;
	let mut previous_sum = values[0] + values[1] + values[2];

	while i < values.len() {
		let sum = values[i] + values[i-1] + values[i-2];

		if sum > previous_sum {
			counter += 1;
		}

		i += 1;
		previous_sum = sum;
	}

	println!("count = {}", counter);
}
