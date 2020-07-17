use std::env;

fn guess_z_string(size: usize, arr: &str) {
	let v: Vec<i32> = arr.split_whitespace().map(|c| {
		c.parse::<i32>().unwrap()
	}).collect::<Vec<i32>>();
	assert_eq!(v.len(), size);
	if v[0] != 0 {
		println!("!");
		return;
	}
	let mut start = 'a' as u8;
	let mut new_s = String::new();
	new_s.push(start as char);
	let mut i = 1;
	while i < size {
		if v[i] == 0 {
			new_s.push((start + 1) as char);
			start += 1;
			i += 1;
			continue;
		}
		let n = new_s.len();
		let mut passed = 0;
		for j in 0..i {
			if j >= n || passed >= v[i] as usize {
				break;
			}
			new_s.push(new_s.chars().nth(j).unwrap());
			passed += 1;
		}
		// let tmp = new_s[0..i].to_string();
		// for j in 0..i {
		// 	tmp.bytes().for_each(|c| {
		// 		new_s.push(c as char);
		// 	});
		// }
		i += passed;
	}
	println!("{}", new_s);
}

fn for_any_string(arr: Vec<&str>) {
	let mut iter = arr.into_iter();
	let mut arg: Option<&str> = iter.next();
	while let Some(s) = arg {
		let n = match s.parse::<usize>() {
			Ok(n) => {
				if n <= 0 { return; }
				n
			},
			Err(_) => return,
		};
		match iter.next() {
			Some(array) => {
				guess_z_string(n, array);
			},
			None => return,
		};
		arg = iter.next();
	}
}

fn main() {
	let mut n: usize; // useless yet
	let mut it = env::args().into_iter();
	let mut splitted: std::str::SplitTerminator<char>;
	let c = match it.nth(1) {
		Some(arg) => arg,
		None => return,
	};
	splitted = c.split_terminator('\n');
	let tmp = splitted.next().unwrap();
	n = match tmp.parse::<usize>() {
		Ok(d) => {
			if d == 0 {
				println!("d is 0");
				return;
			}
			d
		},
		Err(_) => return,
	};
	let a: Vec<&str> = splitted.collect::<Vec<&str>>();
	if let 0 = a.len() {
		println!("a.len() is 0");
		return;
	}
	for_any_string(a);
}
