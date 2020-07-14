use std::env;

// not efficient algos

//checks fixed size potential palindrom
fn yes_palindrom(s: &str, len: usize) -> Option<usize> {
	//println!("len of substr: {}", len);
	if len == 1 {
		return Some(1);
	}
	for i in 0..len {
		if i == len-1-i || i > len-1-i {
			break;
		}
		// println!("{}", s);
		// println!("len-1-i: {}", len-1-i);
		// println!("s.chars().nth(i) = {}, i: {}", s.chars().nth(i).unwrap(), i);
		// println!("s.chars().nth(len-1-i) = {}, len-1-i: {}", s.chars().nth(len-1-i).unwrap(), len-1-i);
		if s.chars().nth(i).unwrap() == s.chars().nth(len-1-i).unwrap() {
			//println!("s.chars().nth({}) and s.chars().nth({}) are equ", i, len-1-i);
			// if i == len-i || i+1 == len-i {
			// 	break;
			// }
		}
		else {
			return Some(1);
		}
	}
	Some(len)
}

fn search_print(s: &str) {
	let n = s.len();
	let mut max: usize = 0;
	//println!("len of str: {}", n);

	for i in 0..n {
		//println!("i is s[{}]", i);
		for j in (i..n).rev() {
			//println!("j is s[{}]", j);
			if s.chars().nth(i) == s.chars().nth(j) {
				//println!("both s[{}] and s[{}] are equ", i, j);
				if let Some(len) = yes_palindrom(&s[i..j+1], j+1-i) {
					if len > max {
						max = len;
					}
				}
			}
		}
	}
	println!("{}", max);
}

fn for_any_string(arr: Vec<String>) {
	for (i, s) in arr.iter().enumerate() {
		println!("string #{}", i);
		search_print(s);
	}
}

fn main() {
	let mut n: usize = 0;
	let mut it = env::args().into_iter();
	n = match it.nth(1) {
		Some(c) => {
			match c.parse::<usize>() {
				Ok(d) => d,
				Err(_) => n,
			}
		}
		None => n,
	};
	if n == 0 {
		println!("n is 0");
		return;
	}
	// let a: Vec<String> = env::args().next().into_iter().next().into_iter().collect();
	let a: Vec<String> = it.collect();
	if let 0 = a.len() {
		println!("a.len() is 0");
		return;
	}
	for_any_string(a);
	// for i in a.iter() {
	// 	println!("{}", i);
	// }
}
