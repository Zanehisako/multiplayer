use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    //127.0.0.1 is the IP address, which typically refers to the local machine (localhost).
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buffer = [0; 1024];
    loop {
        //This method waits for data to arrive and then returns the data along with the sender's address
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        println!("{:?} \ncoming from{:?}", len, addr);

        //Since UDP is connectionless, the socket doesn't keep track of who it's sending data to—you must specify the address each time
        let len = socket.send_to(&buffer[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
    Ok(())
}
