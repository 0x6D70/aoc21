use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;

#[derive(Debug)]
struct Vent {
	x1 : usize,
	y1 : usize,
	x2 : usize,
	y2 : usize
}

fn main() {
	let file = File::open("input.txt").unwrap();

	let mut lines: Vec<String> = Vec::new();

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();
	
		lines.push(line);	
	}

	let mut vents : Vec<Vent> = Vec::new();

	for line in lines {
		let values : Vec<usize> = line.split(&['-', '>', ','][..]).filter_map(|value| {
			if value.is_empty() {
				return None;
			}

			return value.trim().parse::<usize>().ok();
		}).collect();
	
		vents.push(Vent{
			x1: values[0],
			y1: values[1],
			x2: values[2],
			y2: values[3]
		});

	}

	let mut max_x = 0;
	let mut max_y = 0;

	for vent in &vents {
		max_x = cmp::max(max_x, cmp::max(vent.x1, vent.x2));
		max_y = cmp::max(max_y, cmp::max(vent.y1, vent.y2));	
	}

	max_x += 1;
	max_y += 1;

	println!("max_x = {}\nmax_y = {}", max_x, max_y);

	let mut arr = vec![Vec::<i32>::new(); max_y as usize];

	for elem in &mut arr {
		let mut i = 0;

		while i < max_x {
			elem.push(0);
			i += 1;
		}
	}

	for vent in &vents {
		if vent.x1 == vent.x2 {
			// y is changing
			let x = vent.x1 as usize;
			let mut y_start = cmp::min(vent.y1, vent.y2) as usize;
			let y_end = cmp::max(vent.y1, vent.y2) as usize;

			while y_start <= y_end {
				arr[y_start][x] += 1;
				y_start += 1;
			}
		} else if vent.y1 == vent.y2 {
			// x is changing
			let y = vent.y1 as usize;
			let mut x_start = cmp::min(vent.x1, vent.x2) as usize;
			let x_end = cmp::max(vent.x1, vent.x2) as usize;

			while x_start <= x_end {
				arr[y][x_start] += 1;
				x_start += 1;
			}	
		} else {
			// uncomment for solution one
			// continue;
		
			let mut x1 = vent.x1 as usize;
			let mut y1 = vent.y1 as usize;

			arr[y1][x1] += 1;

			while x1 != vent.x2 && y1 != vent.y2 {
				if x1 < vent.x2 {
					x1 += 1;
				} else {
					x1 -= 1;
				}

				if y1 < vent.y2 {
					y1 += 1;
				} else {
					y1 -= 1;
				}				

				arr[y1][x1] += 1;
			}	
		}
	}

	let mut counter = 0;

	for y in &arr {
		for x in y {
			if *x >= 2 {
				counter += 1;
			}
		}
	}

	println!("result = {}", counter);
}
