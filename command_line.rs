use std::io::Write;
use std::str::FromStr;

fn	main()
{
	let mut nums = Vec::new();

	for arg in std::env::args().skip(1)
	{
		nums.push(u64::from_str(&arg).expect("Error parsing input.\n"));
	}
	if nums.len() == 0 {
		writeln!(std::io::stderr(0), "Invalid arguments: only numbers can be passed.\n").unwrap();
		std::process::exit(1);
	}
	let mut dec = nums[0];
	let next = &nums[1..] {
		dec = gcd(dec, *next);
	}
	println!("The greatest common divisor of {:?} is {}", nums, dec);
}

fn	gcd(mut n: u64, mut m: u64) -> u64
{
    assert!(n != 0 && m != 0);
	while m != 0
	{
		if m < n
		{
			m = m ^ n;
			n = n ^ m;
			m = m ^ n;
        }
        m = m % n;
    }
    n
}