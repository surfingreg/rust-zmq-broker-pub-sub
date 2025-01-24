//! 
//! broker/main.rs
//! 

fn main() {
	let context = zmq::Context::new();
	let socket_in = context.socket(zmq::SUB).unwrap();
	socket_in.bind("tcp://*:5556").expect("could not connect to publisher");
	
	let socket_out = context.socket(zmq::PUB).unwrap();
	socket_out.bind("tcp://*:5557").unwrap();	

	let topic_str = "test_topic".to_string();
	let subscription = topic_str.clone().into_bytes();
	socket_in.set_subscribe(&subscription).unwrap();

	loop {
		let topic = socket_in.recv_msg(0).unwrap();
		let data = socket_in.recv_msg(0).unwrap();
		assert_eq!(&topic[..], &subscription[..]);
		println!("[broker] {}", std::str::from_utf8(&data).unwrap());
		
		socket_out.send(&topic_str, zmq::SNDMORE).unwrap();
		socket_out.send(data, 0).unwrap();
	}
}