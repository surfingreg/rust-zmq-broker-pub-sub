//! 
//! edge/main.rs
//! 

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB).unwrap();
    socket.connect("tcp://localhost:5557").expect("could not connect to publisher");

    let topic = "test_topic".to_string();
    let subscription = topic.into_bytes();
    socket.set_subscribe(&subscription).unwrap();

    loop {
        
        //let m = socket.recv_string(0).unwrap().unwrap();
        //println!("[edge] {}", m);
        
        let topic = socket.recv_msg(0).unwrap();
        let data = socket.recv_msg(0).unwrap();
        assert_eq!(&topic[..], &subscription[..]);
        println!("[edge] {}", std::str::from_utf8(&data).unwrap());
    }
}