use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buffer = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let len = socket.send_to(&buffer[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
    Ok(())
}
