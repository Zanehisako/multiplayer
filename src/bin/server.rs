use std::net::SocketAddr;

use rand::Rng;
use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    //127.0.0.1 is the IP address, which typically refers to the local machine (localhost).
    let socket_private = UdpSocket::bind("127.0.0.1:8080").await?;
    let socket_broadcast = UdpSocket::bind("0.0.0.0:8080").await?;
    socket_broadcast.set_broadcast(true)?;
    let broadcast_address: SocketAddr = "255.255.255.255:8080".parse().unwrap();
    let mut buffer = [0; 4];
    let key = rand::thread_rng().gen_range(0..1000);
    println!("the key {:?}", key);

    loop {
        //This method waits for data to arrive and then returns the data along with the sender's address
        let (len, addr) = socket_private.recv_from(&mut buffer).await?;
        let msg: u32 = match String::from_utf8_lossy(&buffer[..len]).trim().parse() {
            Ok(result) => result,
            Err(_) => 0,
        };

        println!("{:?} coming from {:?}", msg, addr);

        if msg == key {
            println!("sending a message");
            socket_broadcast
                .send_to(b"Congratulations ", broadcast_address)
                .await?;
            println!("done sending a message");
        } else if msg > key {
            socket_private.send_to(b"Lower", addr).await?;
        } else {
            socket_private.send_to(b"Higher", addr).await?;
        }

        //Since UDP is connectionless, the socket doesn't keep track of who it's sending data to—you must specify the address each time
    }
}
