use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Token, Ready, PollOpt, Events};
use std::collections::HashMap;
use std::io::{Read, Write};

static RESPONSE: &str = "HTTP/1.1 200 OK
Content-Type: text/html
Connection: keep-alive
Content-Length: 6
hello
";
//"HTTP/1.1 200 OK\n\rContent-Type: text/html\n\rConnection: keep-alive\n\rContent-Length: 6\n\r\nhello\n";

fn main()
{
	let mut counter: usize = 0;

	let mut sockets: HashMap <Token, TcpStream> = HashMap::new();
	let mut requests: HashMap <Token, Vec<u8>> = HashMap::new();

	let address = "0.0.0.0:8080";
	let listener = TcpListener::bind(&address.parse().unwrap()).unwrap();
	let poll = Poll::new().unwrap();
	poll.register(&listener, Token(0), Ready::readable(), PollOpt::edge()).unwrap();
	let mut events = Events::with_capacity(1024);
	loop {
		poll.poll(&mut events, None).unwrap();
		for event in &events
		{
			match event.token()
			{
				Token(0) => {
					loop {
						match listener.accept()
						{
							// std::io::Result<(TcpStream, SocketAddr)>
							Ok((socket, address)) => {
								counter += 1;
								println!("Got connection from {}", address);
								let token = Token(counter);
								poll.register(&socket, token, Ready::readable(), PollOpt::edge()).unwrap();
								//poll.register(&socket, token, Ready::writable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
								sockets.insert(token, socket);
								requests.insert(token, Vec::with_capacity(192));
							},
							Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
							Err(e) => panic!("Unexpected error: {}", e),
						}
					}
				},
				token if event.readiness().is_readable() =>
				{
					loop {
						let mut buffer = [0 as u8; 1024];
						let read = sockets.get_mut(&token).unwrap().read(&mut buffer);
						match read {
							Ok(0) => {
								sockets.remove(&token);
								break;
							},
							Ok(bytes) => {
								println!("Succesfully received {} bytes with token {}", bytes, token.0);
								let req = requests.get_mut(&token).unwrap();
								for byte in &buffer[0..bytes] {
									req.push(*byte);
								};
								// requests.insert(token, buffer.to_vec());
								let ready = requests.get(&token).unwrap().windows(4).find(|window| is_request(*window)).is_some();
								if ready {
									let socket = sockets.get(&token).unwrap();
									poll.reregister(socket, token, Ready::writable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
								}
							},
							Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
							Err(e) => panic!("Unexpected error: {}", e),
						}
					}
				},
				token if event.readiness().is_writable() =>
				{
					//let bytes = response[&token];
					requests.get_mut(&token).unwrap().clear();
					sockets.get_mut(&token).unwrap().write_all(RESPONSE.as_bytes()).unwrap();
					// sockets.remove(&token);
					poll.reregister(sockets.get(&token).unwrap(), token, Ready::readable(), PollOpt::edge()).unwrap();
				},
				_ => unreachable!()
			}
		}
	}
}

fn is_request(window: &[u8]) -> bool
{
	window.len() >= 4 && (window[0] == '\r' as u8) && (window[1] == '\n' as u8)
	&& (window[2] == 'r' as u8) && (window[3] == '\n' as u8)
}