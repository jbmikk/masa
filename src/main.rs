use std::net::UdpSocket;
use std::io;

fn reply() -> Result<(), io::Error> {
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

fn main() {

    match reply() {
        Ok(_) => println!("Replied"),
        Err(x) => println!("Error! {}", x)
    }

    println!("Es una masa!");
}
