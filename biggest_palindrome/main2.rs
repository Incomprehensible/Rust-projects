use std::env;

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
	}
	Some(len)
}

fn search_print(s: &str) {
	let n = s.len();

	for mut j in (0..n).rev() {
		let mut i = 0;
		if j == 0 {
			break;
		}
		while j < n {
			if s.chars().nth(i) == s.chars().nth(j) {
				//println!("both s[{}] and s[{}] are equ", i, j);
				if let Some(len) = yes_palindrom(&s[i..j+1], j+1-i) {
					if len != 1 {
						println!("{}", len);
						return;
					}
				}
			}
			i += 1;
			j += 1;
		}
	}
	println!("{}", 1);
}

fn for_any_string(arr: Vec<&str>) {
	for (i, s) in arr.iter().enumerate() {
		//println!("str #{}", i);
		search_print(s);
	}
}

fn main() {
	if env::args().len() != 2 {
		return;
	}
	let input: String = env::args().into_iter().nth(1).unwrap();
	if input.len() == 0 {
		return;
	}
	let mut it = input.lines();
	let n: usize = match it.nth(0) {
		Some(c) => {
			match c.parse::<usize>() {
				Ok(d) => d,
				Err(_) => 0,
			}
		}
		None => 0,
	};
	if n == 0 {
		return;
	}
	let v: Vec<&str> = it.collect();

	// for i in v.iter() {
	// 	println!("{}", i);
	// }
	for_any_string(v);
}
