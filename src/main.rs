use std::collections::HashMap;
use std::env;
use std::thread;

use chrono::{DateTime, Duration, Utc};


fn main() {
	let mut store: HashMap<String, (DateTime<Utc>, String)> = HashMap::new();

	let args: Vec<String> = env::args().collect();

	let bind_port = &args[1];
	let peer_port = &args[2];
	let command_port = &args[3];

	let zmq_ctx = zmq::Context::new();

	let command_socket = zmq_ctx.socket(zmq::REP).unwrap();
	command_socket.bind(&format!("tcp://*:{}", &command_port)).unwrap();

	let publisher = zmq_ctx.socket(zmq::PUB).unwrap();
	publisher.bind(&format!("tcp://*:{}", &bind_port)).unwrap();

	let subscriber = zmq_ctx.socket(zmq::SUB).unwrap();
	subscriber.connect(&format!("tcp://localhost:{}", &peer_port)).unwrap();
	subscriber.set_subscribe(b"").unwrap();

	println!("Stored {:?}", store);

	loop {
		let now: DateTime<Utc> = Utc::now();

		store.retain(|_, (expiry, _)| { *expiry > now });
		match subscriber.recv_bytes(zmq::DONTWAIT) {
			Ok(message) => {
				println!("Received sync.");
				handle_command(String::from_utf8(message).unwrap(), &mut store, &command_socket, &publisher, true);
			},
			Err(zmq::Error::EAGAIN) => (),
			Err(e) => println!("Error while receiving a message: {}", e),
		};

		match command_socket.recv_bytes(zmq::DONTWAIT) {
			Ok(message) => {
				handle_command(String::from_utf8(message).unwrap(), &mut store, &command_socket, &publisher, false)
			},
			Err(zmq::Error::EAGAIN) => (),
			Err(e) => println!("Error while receiving a message: {}", e),
		};

		thread::sleep(Duration::milliseconds(1000).to_std().unwrap());
	}
}


fn handle_command(message: String, store: &mut HashMap<String, (DateTime<Utc>, String)>, command_socket: &zmq::Socket, publisher: &zmq::Socket, from_peer: bool) {
	let parsed_message: Vec<&str> = message.split_whitespace().collect();

	match parsed_message.as_slice() {
		["SET", key, value, ttl] => {
			let key = key.to_string();
			let valid_until = Utc::now().checked_add_signed(Duration::seconds(ttl.parse::<i32>().unwrap().into())).unwrap();

			println!("Received a SET command.");

			let elem = store.get(&key);

			if let Some((existing_expiry, value)) = elem {
				let value = value.to_string();
				if existing_expiry < &valid_until {
					store.insert(key, (valid_until, value));
				}
			}
			else {
				store.insert(key, (valid_until, value.to_string()));
			}
			
			if !from_peer {
				println!("Sending sync.");
				command_socket.send(&message, 0).unwrap();
				publisher.send(&message, 0).unwrap();
			}

			println!("Stored {:?}", store);
		},
		["GET", key] => {
			println!("Received a GET command.");
			let key = key.to_string();
			let entry = store.get(&key);

			if let Some((expiry, value)) = entry {
				if Utc::now() > *expiry {
					command_socket.send("NO", 0).unwrap();
				}
				else {
					command_socket.send(&format!("OK {}", value), 0).unwrap();
				}
			}
			else {
				command_socket.send("NO", 0).unwrap();
			};
		},
		_ => {
			println!("Received an invalid command.");
			command_socket.send("ERR", 0).unwrap();
		},
	}
}
