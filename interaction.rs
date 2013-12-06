extern mod extra;

use std::rt::io::*;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::println;
use std::cell::Cell;
use std::task;
use std::{os, str, io};

static PORT: int = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count: uint = 0;

fn main() { 
 let socket = net::tcp::TcpListener::bind(SocketAddr {ip: Ipv4Addr(127,0,0,1), port: PORT as u16});
    
    println(fmt!("Listening on tcp port %d ...", PORT));
    let mut acceptor = socket.listen().unwrap();
    
    // we can limit the incoming connection count.
    //for stream in acceptor.incoming().take(10 as uint) {
    for stream in acceptor.incoming() {
        println!("Saw connection!");
        let stream = Cell::new(stream);
        // Start a task to handle the connection
        do task::spawn {
            let mut stream = stream.take();
            let mut buf = [0, ..500];
            stream.write("welcome\n".as_bytes());
            loop {
                stream.read(buf);
                let request_str = str::from_utf8(buf);
                stream.write("Thanks for telling me about ::".as_bytes());
                stream.write(buf);
            }
        }
    }
}
