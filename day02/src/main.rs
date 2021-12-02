use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").unwrap();

	let mut x = 0;
	let mut y = 0;

	let mut aim = 0;
	let mut x2 = 0;
	let mut y2 = 0;

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();

		let v: Vec<&str> = line.as_str().split(' ').collect();

		let number = v[1].parse::<i32>().unwrap();

		if v[0] == "down" {
			y += number;
			aim += number;
		} else if v[0] == "up" {
			y -= number;
			aim -= number;
		} else if v[0] == "forward" {
			x += number;

			x2 += number;
			y2 += number * aim;
		} else {
			panic!("found {}", v[0]);
		}
	}

	println!("result1 = {}", x * y);
	println!("result1 = {}", x2 * y2);
}
