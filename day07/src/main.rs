use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").unwrap();

	let mut lines: Vec<String> = Vec::new();

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();
	
		lines.push(line);	
	}

	let numbers : Vec<usize> = lines[0].split(',').map(|x| x.parse::<usize>().unwrap()).collect();

	let mut min_fuel = std::usize::MAX;
	let mut min_fuel2 = std::usize::MAX;

	for i in 0..10000 {
		let mut fuel = 0;
		let mut fuel2 = 0;

		for num in &numbers {
			let current_fuel;
			if *num < i {
				current_fuel = i - *num;
			} else {
				current_fuel = *num - i;
			}

			fuel += current_fuel;

			let mut sum = 0;
			let mut i = 1;

			while i <= current_fuel {
				sum += i;
				i += 1;
			}

			fuel2 += sum;
		}

		if min_fuel > fuel {
			min_fuel = fuel;
		}

		if min_fuel2 > fuel2 {
			min_fuel2 = fuel2;
		}	
	}

	println!("result = {}", min_fuel);
	println!("result2 = {}", min_fuel2);
}
