#![allow(unused)]
#![allow(dead_code)]
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The Server's logic
    /*
       1. bind address.
       2. accept conn.
       3. using `tcp.read()` to read data into local buffer.
       4. using `tcp.write()` to write data from buffer into conn.
    */
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    let (mut tcp, _) = server.accept().await?;
    let mut buffer = [0u8; 16];
    loop {
        let n = tcp.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        let _ = tcp.write(&buffer[..n]).await?;
    }
    Ok(())
}
