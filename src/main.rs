use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to server");
    
    let orders = vec![
        "BUY NVDA 500 10\n",
        "SELL NVDA 495 5\n",
        "BUY AAPL 150 20\n",
    ];
    
    for order in orders {
        stream.write_all(order.as_bytes()).await.unwrap();
        println!("Sent: {}", order.trim());
        
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        println!("Server says: {}", String::from_utf8_lossy(&buf[..n]));
    }
}    