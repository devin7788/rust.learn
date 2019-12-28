use std::io::prelude::*;
use std::net::TcpStream;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut stream = TcpStream::connect("192.168.10.1:111001").expect("Couldn't connect to the server...");;
    for x in 0..10 {

    }
    Ok(())
}
