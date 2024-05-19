#![allow(unused)]
#![allow(dead_code)]
use futures::{SinkExt, StreamExt};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

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
    loop {
        let (mut tcp, _) = server.accept().await?;
        let (reader, writer) = tcp.split();
        let mut stream = FramedRead::new(reader, LinesCodec::new());
        let mut sink = FramedWrite::new(writer, LinesCodec::new());
        while let Some(Ok(mut msg)) = stream.next().await {
            msg.push_str(" ❤");
            sink.send(msg).await?;
        }
    }
    Ok(())
}
