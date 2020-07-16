fn print_vec(arr: &[i32]) {
	for i in arr.iter() {
		print!("{} ", i);
	}
	println!();
	// println!("{:?}", arr);
}

fn z_function(s: &str, n: usize) -> Vec<i32> {
	let mut arr: Vec<i32> = vec![0];
	for _ in 0..n-1 {
		arr.push(0);
	}
	for i in 1..n {
		let mut j = 0;
		let mut ind = i;
		while ind < n && s.chars().nth(j) == s.chars().nth(ind) {
			ind += 1;
			j += 1;
			arr[i] += 1;
		}
	}
	arr
}

fn main() {
	let s: &str = "abacaba";
	let arr = z_function(&s, s.len());
	print_vec(&arr);
}