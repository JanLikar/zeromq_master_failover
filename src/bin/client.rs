use std::env;
use std::thread;

const NOT_MASTER: u8 = 5;


fn main() {
	let mut store: String = "Initial".to_string();

	let args: Vec<String> = env::args().collect();

	let port1 = &args[1];
	let port2 = &args[2];

	let zmq_ctx = zmq::Context::new();

	let socket = zmq_ctx.socket(zmq::REQ).unwrap();
	socket.connect(&format!("tcp://localhost:{}", &port1));
	socket.connect(&format!("tcp://localhost:{}", &port2));


	socket.send("SET fpppsafpafs", 0);
	socket.recv_msg(0);

	// thread::sleep_ms(5000);

	socket.send("GET", 0);
	println!("{:?}", String::from_utf8(socket.recv_msg(0).unwrap().to_vec()).unwrap());
	socket.send("GET", 0);
	println!("{:?}", String::from_utf8(socket.recv_msg(0).unwrap().to_vec()).unwrap());
	socket.send("GET", 0);
	println!("{:?}", String::from_utf8(socket.recv_msg(0).unwrap().to_vec()).unwrap());
}
