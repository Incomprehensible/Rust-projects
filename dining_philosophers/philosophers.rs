use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher
{
	name : String,
	left: usize,
	right: usize,
}

struct Table
{
	forks: Vec<Mutex<()>>,
}

impl Philosopher {
	fn new(naming: &str, left: usize, right: usize) -> Philosopher
	{
		Philosopher {
			name : naming.to_string(),
			left: left,
			right: right,
		}
	}

	fn eat(&self, table: &Table)
	{
		let _left = table.forks[self.left].lock().unwrap();
		thread::sleep(Duration::from_millis(1500));
		let _right = table.forks[self.right].lock().unwrap();
		println!("{} started!", self.name);
		thread::sleep(Duration::from_millis(1000));
		println!("{} finished!", self.name);
	}
}

fn main() {
	let table = Arc::new(Table {
		forks: vec![
			Mutex::new(()),
			Mutex::new(()),
			Mutex::new(()),
			Mutex::new(()),
			Mutex::new(()),
			Mutex::new(()),
			Mutex::new(()),
		]
	});

	let people = vec![
	Philosopher::new("Kek Cheburek", 0, 1),
	Philosopher::new("Vladimir Putin", 1, 2),
	Philosopher::new("Dmitry Medvedev", 2, 3),
	Philosopher::new("Donald Trump", 4, 3),
	Philosopher::new("Andrey Malahov", 4, 5),
	Philosopher::new("Philip Kirkorov", 5, 6),
	Philosopher::new("Masha Alkasha", 0, 6),
	];

	let handles : Vec<_> = people.into_iter().map(|pers| {
		let table = table.clone();
		thread::spawn(move || {
			pers.eat(&table);
		})
	}).collect();

	for h in handles
	{
		h.join().unwrap();
	}
}
