use std::{collections::HashSet, net::SocketAddr, sync::Arc};

use rand::Rng;
use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    //127.0.0.1 is the IP address, which typically refers to the local machine (localhost).
    let socket = std::sync::Arc::new(UdpSocket::bind("127.0.0.1:8888").await.unwrap());
    let key = rand::thread_rng().gen_range(0..1000);
    println!("the key {:?}", key);

    let socket_recive = socket.clone();
    let clients: std::sync::Arc<tokio::sync::Mutex<HashSet<SocketAddr>>> =
        std::sync::Arc::new(tokio::sync::Mutex::new(HashSet::new()));
    //recives the messages and broadcast to the channel
    let client_clone = Arc::clone(&clients);
    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            //This method waits for data to arrive and then returns the data along with the sender's address
            let (len, addr) = socket_recive.recv_from(&mut buffer).await.unwrap();
            if !client_clone.lock().await.contains(&addr) {
                client_clone.lock().await.insert(addr);
            }
            let msg: u32 = match String::from_utf8_lossy(&buffer[..len]).trim().parse() {
                Ok(result) => result,
                Err(_) => 0,
            };

            println!("{:?} coming from {:?}", msg, addr);

            if msg == key {
                println!("sending a message");
                for client in clients.lock().await.iter() {
                    socket_recive
                        .send_to(format!("Congratulations {}", addr).as_bytes(), *client)
                        .await
                        .unwrap();
                }
                println!("done sending a message");
            } else if msg > key {
                socket_recive.send_to(b"Lower", addr).await.unwrap();
            } else {
                socket_recive.send_to(b"Higher", addr).await.unwrap();
            }

            //Since UDP is connectionless, the socket doesn't keep track of who it's sending data to—you must specify the address each time
        }
    })
    .await
    .unwrap();
    Ok(())
}
