use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").unwrap();

	let mut v: Vec<String> = Vec::new();

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();

		v.push(line);
	}

	let mut gamma = 0;
	let mut epsilon = 0;

	for i in 0..12 {
		let mut counter = 0;

		for line in &v {
			if line.chars().nth(i).unwrap() == '0' {
				counter -= 1;
			} else if line.chars().nth(i).unwrap() == '1' {
				counter += 1;
			} else {
				panic!("invalid character found: {}", line.chars().nth(i).unwrap());
			}
		}

		gamma = gamma << 1;
		epsilon = epsilon << 1;

		if counter > 0 {
			gamma += 1;
		} else if counter < 0 {
			epsilon += 1;
		}
	}

	println!("result = {}", gamma * epsilon);

	let mut v_oxygen = v.clone();

	for i in 0..12 {
		if v_oxygen.len() == 1 {
			break;
		}

		let mut counter = 0;

		for line in &v_oxygen {
			if line.chars().nth(i).unwrap() == '0' {
				counter -= 1;
			} else if line.chars().nth(i).unwrap() == '1' {
				counter += 1;
			}
		}

		let del : char;

		if counter >= 0 {
			del = '1';
		} else {
			del = '0';
		}

		v_oxygen = v_oxygen
			.into_iter()
			.filter(|line| line.chars().nth(i).unwrap() == del)
			.collect();
	}

	let oxygen = isize::from_str_radix(&v_oxygen[0], 2).unwrap();

	let mut v_co2 = v.clone();

	for i in 0..12 {
		if v_co2.len() == 1 {
			break;
		}

		let mut counter = 0;

		for line in &v_co2 {
			if line.chars().nth(i).unwrap() == '0' {
				counter -= 1;
			} else if line.chars().nth(i).unwrap() == '1' {
				counter += 1;
			}
		}

		let del : char;

		if counter < 0 {
			del = '1';
		} else {
			del = '0';
		}

		v_co2 = v_co2
			.into_iter()
			.filter(|line| line.chars().nth(i).unwrap() == del)
			.collect();
	}

	let co2 = isize::from_str_radix(&v_co2[0], 2).unwrap();

	println!("result2 = {}", oxygen * co2);
}
