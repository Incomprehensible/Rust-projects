use std::env;

//checks fixed size potential palindrom
fn yes_palindrom(s: &str, len: usize) -> Option<usize> {
	println!("len of substr: {}", len);
	if len == 1 {
		return Some(1);
	}
	for i in 0..len {
		if i == len-1-i || i+1 == len-1-i {
			break;
		}
		println!("s.chars().nth(i) = {}", s.chars().nth(i).unwrap());
		println!("s.chars().nth(len-i) = {}", s.chars().nth(len-1-i).unwrap());
		if s.chars().nth(i).unwrap() == s.chars().nth(len-1-i).unwrap() {
			println!("s.chars().nth({}) and s.chars().nth({}) are equ", i, len-1-i);
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
	println!("len of str: {}", n);
	let mut min = 0;
	let mut max = 0;
	let mut len: usize = 0;

	for i in 0..n {
		println!("i is s[{}]", i);
		for j in (i..n).rev() {
			println!("j is s[{}]", j);
			if s.chars().nth(i) == s.chars().nth(j) {
				println!("both s[{}] and s[{}] are equ", i, j);
				if let Some(tmp) = yes_palindrom(&s[i..j], j-i) {
					println!("tmp: {}", tmp);
					if tmp > len {
						min = i;
						max = j;
						len = tmp;
					}
				}
			}
		}
	}
	println!("len: {}, start: {}, end: {}", len, min, max);
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
