extern crate num;
use num::Complex;

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

extern crate crossbeam;

fn	encode_png(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> //Result<()>
{
	let outputimg = File::create(filename)?;
	// let output = match File::create(filename) {
	// 	Ok(f) => { f }
	// 	Err(e) => { return Err(e); }
	// };
	let encoder = PNGEncoder::new(outputimg);

	encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
	Ok(())
}

fn	escape_chance(c: Complex <f64>, limit: u32) -> Option<u32>
{
	let mut z = Complex { re: 0.0, im: 0.0 };
	for i in 0..limit
	{
		z = z * z + c;
		if z.norm_sqr() > 4.0
		{
			return Some(i)
		}
	}
	None
}

fn	pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex <f64>, lower_right: Complex <f64>) -> Complex <f64>
{
	let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
	Complex
	{
		re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
		im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
	}
}

fn	render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>)
{
	assert!(pixels.len() == bounds.0 * bounds.1);

	for row in 0 .. bounds.1
	{
		for column in 0 .. bounds.0
		{
			let point = pixel_to_point(bounds, (row, column), upper_left, lower_right);
			pixels[row * bounds.0 + column] = match escape_chance(point, 255)
			{
				None => 0,
				Some(n) => 255 - n as u8
			};
		}
	}
}

fn	parse_coordinates<T: FromStr>(s: &str, sep: char) -> Option<(T, T)>
{
	match s.find(sep)
	{
		None => None,
		Some(index) =>
		{
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1..]))
			{
				//Some(Ok(x), Ok(y)) => Some((x, y)),
				//Some((Ok(x), Ok(y))) => Some((x, y)),
				(Ok(x), Ok(y)) => Some((x, y)),
				_ => None
			}
		}
	}
}

#[test]
fn	test_parse()
{
	assert_eq!(parse_coordinates::<i32>("kek,kek", ','), None);
	assert_eq!(parse_coordinates::<i32>("1,", ','), None);
	assert_eq!(parse_coordinates::<i32>("255x666", 'x'), None);
	assert_eq!(parse_coordinates::<f64>("255x777", 'x'), None);
	assert_eq!(parse_coordinates::<f64>("255.0:777", ':'), Some((255.0, 777.0)));
	assert_eq!(parse_coordinates::<f64>("25.50^77.07", '^'), Some((25.5, 77.07)));
}

fn	fetch_coordinates(s: &str, c: char) -> Option<Complex<f64>>
{
	match parse_coordinates(s, c)
	{
		Some((x, y)) => Some(Complex { re: x, im: y}),
		//Some((re, im)) => Some(Complex { re, im }),
		None => None
	}
}

#[test]
fn	test_fetch()
{
	assert_eq!(fetch_coordinates("1.25,-0.0625", ','), Complex <f64> { re: 1.25, im: -0.0625 });
	assert_eq!(fetch_coordinates(",-0.0625", ','), None);
}

fn	main()
{
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 5
	{
		writeln!(std::io::stderr(), "{}: Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT\n", args[0]).unwrap();
		writeln!(std::io::stderr(), "Example: {} mandel.png 1000x650 -1.20,0.47 -1,0.25", args[0]).unwrap();
		std::process::exit(1);
	}
	let bounds = parse_coordinates(&args[2], 'x').expect("Error parsing image dimensions");
	let upper_left = fetch_coordinates(&args[3], ',').expect("Error parsing upper left corner point");
	let lower_right = fetch_coordinates(&args[4], ',').expect("Error parsing lower right corner point");

	let mut pixels = vec![0; bounds.0 * bounds.1];
	//render(&mut pixels, bounds, upper_left, lower_right); // only 1 thread
	//multiple threads
	let threads = 8;
	let rows_per_split = bounds.1 / threads + 1;
	{
		let splits: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_split * bounds.0).collect();
		crossbeam::scope(|spawner|
		{
			for (i, split) in splits.into_iter().enumerate()
			{
				let top = rows_per_split * i;
				let height = split.len() / bounds.0;
				let split_bounds = (bounds.0, height);
				let split_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
				let split_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

				spawner.spawn(move || {
					render(split, split_bounds, split_upper_left, split_lower_right);
				});
			}
		});
	}
	encode_png(&args[1], &pixels, bounds).expect("Error encoding PNG file");
}
