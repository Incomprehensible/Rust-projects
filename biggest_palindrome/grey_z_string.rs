use std::env;

fn find_z_for_arr(s: &str, ind: usize)
{
	println!("DB s: {}, i: {}", s, ind);
	if s.len() < 1 || ind >= s.len() {
		return;
	}
	if ind == 0 {
		println!("{}", 0);
		return;
	}
	let mut z: Vec<usize> = vec![0];
	for _ in 0..s.len() {
		z.push(0);
	}
	for i in ind..s.len() {
		let i = ind;
		while (z[i] + i < s.len()) 
			&& ((s.chars().nth(z[i] + i)) == (s.chars().nth(z[i]))) {
				z[i] += 1;
		}
	}
	println!("{:?}", z);
}

fn find_z_for(s: &str, ind: usize)
{
	//println!("DB s: {}, i: {}", s, ind);
	if s.len() < 1 || ind >= s.len() {
		return;
	}
	if ind == 0 {
		println!("{}", 0);
		return;
	}
	let mut z = 0;
	let i = ind;
	while (z + i < s.len()) 
		&& ((s.chars().nth(z + i)) == (s.chars().nth(z))) {
			z += 1;
	}
	println!("{}", z);
}

const ALPHA: u8 = 26;

fn generate_grey(k: u8) -> String {
	//println!("DB im here");
	assert!(k <= ALPHA);
	if k == 0 {
		//println!("DB k is 0");
		return String::new();
	}

	let start = 'a' as u8;
	let mut new_s = String::new();
	for i in 1..k+1 {
		let mut j = 0;
		if new_s.len() != 0 {
			j = new_s.len();
			let s = new_s.as_str().to_owned();
			s.bytes().for_each(|c| { new_s.push(c as char); });
		}
		new_s.insert(j, (start + i-1) as char);
		//println!("#{} {}", i, new_s);
		// println!("DB {}", (start + i-1) as char);
		// new_s.push((start + i-1) as char);
	}
	new_s
}

fn for_any_string(arr: Vec<&str>) {
	for (i, s) in arr.iter().enumerate() {
		//println!("DB string #{}", i);
	
		let v: Vec<i8> = s.split_whitespace()//.into_iter()
			.map(|x: &str| { 
				match x.parse::<i8>() {
					Ok(n) => n,
					Err(_) => -1,
				} 
			})
			.collect::<Vec<i8>>();
	
		assert_eq!(v.len(), 2);
		// k = v[0]
		// i = v[1]
		assert!(v[0] >= 0 && v[1] >= 0);
		let generated = generate_grey(v[0] as u8);
		find_z_for(&generated, v[1] as usize);
	}
}

fn main() {
	let mut n: usize;
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
