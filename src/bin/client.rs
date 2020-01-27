use dialoguer::Input;

use std::env;
use std::thread;

const NOT_MASTER: u8 = 5;


fn main() {
	let mut store: String = "Initial".to_string();

	let args: Vec<String> = env::args().collect();

	let port1 = &args[1];
	let port2 = &args[2];

	let zmq_ctx = zmq::Context::new();

	let socket1 = zmq_ctx.socket(zmq::REQ).unwrap();
	socket1.connect(&format!("tcp://localhost:{}", &port1));
	socket1.set_rcvtimeo(3000).unwrap();
	socket1.set_sndtimeo(3000).unwrap();

	let socket2 =zmq_ctx.socket(zmq::REQ).unwrap();
	socket2.connect(&format!("tcp://localhost:{}", &port2));
	socket2.set_rcvtimeo(3000).unwrap();
	socket2.set_sndtimeo(3000).unwrap();

	let sockets = [socket1, socket2];

	loop {
		let input = Input::<String>::new().with_prompt(">>> ").interact().unwrap();

		for socket in sockets.iter() {
			match socket.send(&input, 0) {
				Ok(_) => {
					match socket.recv_msg(0) {
						Ok(m) => {
							println!("{:?}", String::from_utf8(m.to_vec()).unwrap());
							break;
						},
						Err(_) => (),
					};
				},
				Err(_) => (),
			}
		}
	}
}
