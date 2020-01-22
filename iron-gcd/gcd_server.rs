extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;


fn	main() {
	let mut router = Router::new();

	router.get("/", get_form, "root");
	router.post("/gcd", post_gcd, "gcd");

	println!("Serving on http://localhost:3000...");
	Iron::new(router).http("localhost:3000").unwrap();
}

fn	get_form(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(r#"
	<title>Gcd_calculator</title>
	<form action="/gcd" method="post">
		<input type="text" name="n"/>
		<input type="text" name="n"/>
		<button type="submit">Compute GCD</button>
	</form>
	"#);

	Ok(response)
}

fn	post_gcd(request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();

	let form_data = match request.get_ref::<UrlEncodedBody>()
	{
		Err(e) => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Error parsing from data {:?}\n", e));
			return Ok(response);
		}
		Ok(map) => map
	};
	let unparsed_nums = match form_data.get("n") {
		None => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Data empty of required parameters\n"));
			return Ok(response);
		}
		Some(nums) => nums
	};
	let mut numbers = Vec::new();
	for num in unparsed_nums {
		match u64::from_str(&num) {
			Err(_) => {
				response.set_mut(status::BadRequest);
				response.set_mut(format!("Value passed is not a number: {:?}\n", num));
				return Ok(response);
			}
			Ok(n) => { numbers.push(n); }
		}
	};
	let mut dec = numbers[0];
	for next in &numbers[1..] {
		dec = gcd(dec, *next);
	}
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(format!("The greatest common divisor of the numbers {:?} is <b>{}</b>", numbers, dec));
	Ok(response)
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