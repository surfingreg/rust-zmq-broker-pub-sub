//! 
//! publisher/main.rs
//! 
//! 

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB).unwrap();
    socket.connect("tcp://localhost:5556").expect("could not bind publisher socket");

    // Ensure subscriber connection has time to complete
    sleep(Duration::from_millis(1000));

    let topic = "test_topic";
    loop {
        let t = std::time::SystemTime::now();
        let msg_out = format!("time: {:?}",t);
        println!("[client] sending: {}", &msg_out);
        
        socket.send(topic, zmq::SNDMORE).unwrap();
        socket.send(&msg_out, 0).unwrap();
        sleep(Duration::from_millis(1000));
    }
}