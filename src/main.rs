#![allow(unused)]
#![allow(dead_code)]

use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The Server's logic
    /*
       1. bind address.
       2. use tokio::spwan to accept conn concurrently.
    */
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    loop {
        let (tcp, _) = server.accept().await?;
        // spawn a separate task for
        // to handle every connection
        tokio::spawn(handle_user(tcp));
    }
}

/// using `tcp.read()` to read data into local buffer.
/// using `tcp.write()` to write data from buffer into conn.s
async fn handle_user(mut tcp: TcpStream) -> anyhow::Result<()> {
    let (reader, writer) = tcp.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    while let Some(Ok(mut msg)) = stream.next().await {
        if msg.starts_with("/quit") {
            break;
        } else {
            msg.push_str(" ❤️");
            sink.send(msg).await?;
        }
    }
    Ok(())
}
