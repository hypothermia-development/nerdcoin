#![allow(warnings)]
use std::net::TcpStream;
use std::thread;
use serde_json::Value;
use serde_json::json;
use std::io::{self, Write};
use std::io::{Read};
use std::time::Duration;
use rand::Rng;

/*

Node code

Code for all the nodes


*/


// All needs testing, no syntax errors, if it works its ready for beta


fn init(self_ip: &str, processor_ip: &str, self_name: &str)
{
    // Sends info of node to processor
    let mut stream = TcpStream::connect(processor_ip);

    let con = json!({
        "Node Name": self_name,
        "Node IP": self_ip,

    });

    let init_ = con.to_string();
    stream.as_mut().expect("Failed to initalize").write_all(init_.as_bytes()).unwrap();
    
}


fn mine(mut stream: std::net::TcpStream, self_name: &str)
{
    let mut rng = rand::thread_rng();


    let mut nerdcoin_gotten = rng.gen_range(0..10);

    let bel = json!({
        "Node: ": self_name,
        "Nerdcoin gotten: ": nerdcoin_gotten,
    });

    let returnable = bel.to_string();

    stream.write_all(returnable.as_bytes()).unwrap();
}



fn main() {

    // Include ports with these IPS (i.e 192.168.0.38:7878)
    let PROCESSOR_IP: &str = "192.168.0.38:7878";
    let SELF_IP: &str = "192.168.0.23:7878";
    let SELF_NAME: &str = "NODE N8U";

    // Initialize the relationship between the node and processor (the node makes the first move)
    init(SELF_IP, PROCESSOR_IP, SELF_NAME);

    loop
    {
        let mut stream = TcpStream::connect(PROCESSOR_IP);

        mine(stream.expect("Failed to mine"), SELF_NAME);
        thread::sleep(Duration::from_secs(3));
    }


    println!("Hello, world!");
}
