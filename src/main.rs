use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// warp
use warp::Filter;

// constants
const WARP_PORT:u16 = 8000;
const TUNG_PORT:i32 = 8001;

// our tcp helper function
async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("o-oh emm gee!! i-i'm getting a message f-from {}-san!!", addr);

    // we don't need the peer map i'm just confused
}

#[tokio::main]
async fn main() {
    // begin -- warp
    println!("running.");

    // setting up filters
    let base = warp::path::end()
        .and(warp::fs::file("pages/index.html"));

    let media = warp::path("f").and(warp::fs::dir("pages/"));

    // i am an expert in the package i hate
    let routes = warp::get().and(
        base
        .or(media)
        );

    tokio::spawn(
        warp::serve(routes)
        .run(([127,0,0,1], WARP_PORT)) // was .await
    );

    // now on to tungstenite
    
    let try_socket = TcpListener::bind(format!("127.0.0.1:{TUNG_PORT}")).await;

    let listener = try_socket.expect("Failed to bind");
    println!("Listening. to tcp, even: 127.0.0.1:{TUNG_PORT}");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}
