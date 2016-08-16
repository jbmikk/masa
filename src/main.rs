use std::net::UdpSocket;
use std::io;
use std::env;

fn listen() -> Result<(), io::Error> {
    let socket = try!(UdpSocket::bind("127.0.0.1:34254"));

    // read from the socket
    let mut buf = [0; 10];

    //let (amt, src) = try!(socket.recv_from(&mut buf));
    let (amt, src) = try!(socket.recv_from(&mut buf));

    // send a reply to the socket we received data from
    let buf = &mut buf[..amt];
    buf.reverse();
    try!(socket.send_to(buf, &src));

    return Ok(());
}

fn broadcast() -> Result<(), io::Error> {
    let socket = try!(UdpSocket::bind("127.0.0.1:34255"));
    
    try!(socket.set_broadcast(true));

    let buf = [1,2,3,4,5,6,7,8,9];

    try!(socket.send_to(&buf, "127.255.255.255:34254"));

    return Ok(());
}

fn unknown() -> Result<(), io::Error> {
    println!("Unknown command");
    return Ok(());
}

fn run_command(command: &str) {

    println!("Command: {}", command);
    let result = match command {
        "listen" => listen(),
        "broadcast" => broadcast(),
        _ => unknown()
    };

    match result {
        Ok(_) => println!("OK"),
        Err(x) => println!("Error! {}", x)
    }
}

fn main() {

    if let Some(command) = env::args().nth(1) {
        run_command(&command);
    } else {
        println!("Es una masa!");
    }

}
