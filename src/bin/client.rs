use std::io::{self, Read, Write};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UdpSocket,
};

async fn recive_message(socket: UdpSocket) {
    let mut buffer: [u8; 1024] = [0; 1024];

    let (len, _) = socket.recv_from(&mut buffer).await.unwrap();
    println!("{:?} ", String::from_utf8_lossy(&buffer[..len]));
}

#[tokio::main]
async fn main() -> io::Result<()> {
    //127.0.0.1 is the IP address, which typically refers to the local machine (localhost).
    let socket_private = std::sync::Arc::new(UdpSocket::bind("127.0.0.1:0").await?);
    let rec = socket_private.clone();
    let send = socket_private.clone();
    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            match rec.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    println!(
                        "Received {} bytes from {}: {:?}",
                        len,
                        addr,
                        String::from_utf8_lossy(&buf[..len])
                    );
                }
                Err(e) => eprintln!("Failed to receive: {}", e),
            }
        }
    });

    loop {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin).lines();
        while let Some(line) = reader.next_line().await? {
            if let Err(e) = send.send_to(line.as_bytes(), "127.0.0.1:8080").await {
                eprintln!("error sending the message {}", e);
            } else {
                println!("success sending {}", line);
            }
        }
    }
}
