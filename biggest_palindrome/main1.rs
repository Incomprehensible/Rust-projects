use std::env;

// not efficient algos

//checks fixed size potential palindrom
fn yes_palindrom(s: &str, len: usize) -> Option<usize> {
	if len == 1 {
		return Some(1);
	}
	for i in 0..len {
		if i == len-1-i || i > len-1-i {
			break;
		}
		if s.chars().nth(i).unwrap() != s.chars().nth(len-1-i).unwrap() {
			return Some(1);
	}
	Some(len)
}

fn search_print(s: &str) {
	let n = s.len();
	let mut max: usize = 0;

	for i in 0..n {
		for j in (i..n).rev() {
			if s.chars().nth(i) == s.chars().nth(j) {
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
		// println!("string #{}", i);
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
		//println!("n is 0");
		return;
	}
	let a: Vec<String> = it.collect();
	if let 0 = a.len() {
		//println!("a.len() is 0");
		return;
	}
	for_any_string(a);
}
