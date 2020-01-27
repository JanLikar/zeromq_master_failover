use std::env;
use std::thread;

const NOT_MASTER: u8 = 5;


fn main() {
	let mut store: String = "Initial".to_string();

	let args: Vec<String> = env::args().collect();

	let bind_port = &args[1];
	let peer_port = &args[2];
	let command_port = &args[3];

	let zmq_ctx = zmq::Context::new();

	let command_socket = zmq_ctx.socket(zmq::REP).unwrap();
	command_socket.bind(&format!("tcp://*:{}", &command_port));

	let publisher = zmq_ctx.socket(zmq::PUB).unwrap();
	publisher.bind(&format!("tcp://*:{}", &bind_port));

	let subscriber = zmq_ctx.socket(zmq::SUB).unwrap();
	subscriber.connect(&format!("tcp://localhost:{}", &peer_port));
	subscriber.set_subscribe(b"");

	loop {
		println!("Stored {:?}", store);

		match subscriber.recv_msg(zmq::DONTWAIT) {
			Ok(m) => {
				store = m.as_str().unwrap().to_string();
				println!("Received sync.", m);
			},
			Err(zmq::Error::EAGAIN) => (),
			Err(e) => println!("Error while receiving a message: {}", e),
		};

		match command_socket.recv_msg(zmq::DONTWAIT) {
			Ok(m) => {
				match std::str::from_utf8(&m[0..3]).unwrap() {
					"SET" => {
						println!("Received a SET command.");
						store = String::from_utf8(m[4..].to_vec()).unwrap();
						command_socket.send("OK", 0);
						publisher.send(&store, 0);
					},
					"GET" => {
						println!("Received a GET command.");
						command_socket.send(&format!("OK {}", store), 0);
					},
					_ => {
						println!("Received an invalid command.");
						command_socket.send("ERR", 0);
					},
				}
			},
			Err(zmq::Error::EAGAIN) => (),
			Err(e) => println!("Error while receiving a message: {}", e),
		};

		thread::sleep_ms(5000);
	}
}
