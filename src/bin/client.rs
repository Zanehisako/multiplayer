use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    //127.0.0.1 is the IP address, which typically refers to the local machine (localhost).
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let mut buffer = [0; 1024];
    let addr = "127.0.0.1:8080";
    loop {
        //Since UDP is connectionless, the socket doesn't keep track of who it's sending data to—you must specify the address each time
        let len = socket.send_to(b"Hello from the other side", addr).await?;
        let (len, _) = socket.recv_from(&mut buffer).await?;
        println!("{:?} ", String::from_utf8_lossy(&buffer[..len]));
    }
    Ok(())
}
