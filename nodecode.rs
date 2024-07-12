#![allow(warnings)]
use std::net::TcpStream;
use std::thread;
use serde_json::Value;
use serde_json::json;
use std::io::{self, Write};
use std::io::{Read};
use std::time::Duration;


/*

Node code

Code for all the nodes


*/



fn init<T>(self_ip: &str, processor_ip: &str, self_name: &str) -> io::Result<TcpStream>
{
    // Sends info of node to processor
    let mut stream = TcpStream::connect(processor_ip);

    let con = json!({
        "Node Name": self_name,
        "Node IP": self_ip,

    });

    let init_ = con.to_string();
    stream.as_mut().expect("Failed to initalize").write_all(init_.as_bytes()).unwrap();

    Ok(stream?)
    
}


fn mine(stream: &mut TcpStream)
{
    println!("Hello");
}



fn main() {

    // Include ports with these IPS (i.e 192.168.0.38:7878)
    let PROCESSOR_IP: &str = "192.168.0.38:7878";
    let SELF_IP: &str = "192.168.0.23:7878";
    let SELF_NAME: &str = "NODE N8U";

    // Initialize the connection with the processor
    let stream = init(SELF_IP, PROCESSOR_IP, SELF_NAME);

    loop
    {
        mine(&stream);
        thread::sleep(Duration::from_secs(3));
    }


    println!("Hello, world!");
}
