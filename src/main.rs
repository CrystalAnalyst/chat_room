#![allow(dead_code)]
#![allow(unused)]
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, *};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

const HELP_MSG: &'static str = include_str!("help.txt");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The Server's logic
    /*
       1. bind address.
       2. use tokio::spwan to accept conn concurrently.
    */
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    let (tx, _) = broadcast::channel::<String>(32);
    loop {
        let (tcp, _) = server.accept().await?;
        // spawn a separate task for
        // to handle every connection
        tokio::spawn(handle_user(tcp, tx.clone()));
    }
}

/// using `tcp.read()` to read data into local buffer.
/// using `tcp.write()` to write data from buffer into conn.s
async fn handle_user(mut tcp: TcpStream, tx: Sender<String>) -> anyhow::Result<()> {
    let (reader, writer) = tcp.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    let mut rx = tx.subscribe();
    sink.send(HELP_MSG).await?;
    loop {
        tokio::select! {
            user_msg = stream.next() => {
                let mut user_msg = match user_msg {
                    Some(msg) => msg?,
                    None => break,
                };
                if user_msg.starts_with("/help") {
                    sink.send(HELP_MSG).await?;
                } else if user_msg.starts_with("/quit") {
                    break;
                } else {
                    user_msg.push_str(" ❤️");
                    let _ = tx.send(user_msg);
                }
            },
            peer_msg = rx.recv() => {
                sink.send(peer_msg?).await?;
            },
        }
    }
    Ok(())
}
