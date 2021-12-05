use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Bingo {
	finished: bool,
	arr : [i32; 25]
}

fn main() {
	let file = File::open("input.txt").unwrap();

	let mut lines: Vec<String> = Vec::new();

	for l in io::BufReader::new(file).lines() {
		let line = l.unwrap();
	
		lines.push(line);	
	}

	let numbers: Vec<i32> = lines[0].split(',').map(|value| value.parse::<i32>().unwrap()).collect();
	let mut bingos: Vec<Bingo> = Vec::new();

	let mut counter = 1;

	while counter < lines.len() {
		if lines[counter].is_empty() {
			counter += 1;
			continue;	
		}

		let mut values : Vec<i32> = Vec::new();	

		for i in 0..5 {
			let mut current_values : Vec<i32> = lines[counter + i]
												.split(' ')
												.filter_map(|value| {
													value.parse::<i32>().ok()
												})
												.collect();

			values.append(&mut current_values);
		}

		counter += 5;

		bingos.push(Bingo {
			finished: false,
			arr: values.try_into().unwrap()
		});
	}

	let mut numbers_count = 4;
	let mut part1_finished = false;

	loop {
		// check if part2 finished
		let mut last_index = 0;
		let mut not_finished_counter = 0;
		let mut i = 0;

		while i < bingos.len() {
			if !bingos[i].finished {
				last_index = i;
				not_finished_counter += 1;
			}

			i += 1;
		}	
		
		if not_finished_counter == 1 {
			let mut sum = 0;

			for k in 0..25 {
				if !vector_contains_value(&numbers, numbers_count, bingos[last_index].arr[k]) {
					sum += bingos[last_index].arr[k];
				}
			}

			println!("{} * {} = {}", sum, numbers[numbers_count], sum * numbers[numbers_count]);

			return;
		}

		// check if some board won
		for mut bingo in &mut bingos {
			for i in 0..5 {
				let start_index = i * 5;
				let mut done = true;
				// check row
				for k in 0..5 {
					if !vector_contains_value(&numbers, numbers_count, bingo.arr[start_index + k]) {
						done = false;
					}
				}
			
				let start_index = i;

				if !done {
					done = true;
	
					// check colum
					for k in 0..5 {
						if !vector_contains_value(&numbers, numbers_count, bingo.arr[start_index + k * 5]) {
							done = false;
						}
					}
				}

				if done {
					let mut sum = 0;
					for k in 0..25 {
						if !vector_contains_value(&numbers, numbers_count, bingo.arr[k]) {
							sum += bingo.arr[k];
						}
					}

					if !part1_finished {
						println!("{} * {} = {}", sum, numbers[numbers_count], sum * numbers[numbers_count]);
						part1_finished = true;				
					}

					bingo.finished = true;
				}	
			}	
		}

		numbers_count += 1;		
	}
}

fn vector_contains_value(v : &[i32], last_index : usize, value : i32) -> bool {
	let mut ret = false;

	let mut i = 0;

	while i < v.len() && i < last_index + 1 {
		if v[i] == value {
			ret = true;
		}

		i += 1;
	}

	ret
}
