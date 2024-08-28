use rand::Rng;
use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    //127.0.0.1 is the IP address, which typically refers to the local machine (localhost).
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buffer = [0; 4];
    let key = rand::thread_rng().gen_range(0..1000);
    println!("the key {:?}", key);

    loop {
        //This method waits for data to arrive and then returns the data along with the sender's address
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        let msg: u32 = String::from_utf8_lossy(&buffer[..len])
            .trim()
            .parse()
            .unwrap();

        println!("{:?} coming from {:?}", msg, addr);

        if msg == key {
            socket.send_to(b"Congratulations ", addr).await?;
        } else {
            socket.send_to(b"wrong try again", addr).await?;
        }

        //Since UDP is connectionless, the socket doesn't keep track of who it's sending data to—you must specify the address each time
    }
}
