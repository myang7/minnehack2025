use std::{
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

// warp
use warp::Filter;

// constants
const WARP_PORT:u16 = 8000;
const TUNG_PORT:i32 = 8001;

// our tcp helper function
async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("o-oh emm gee!! i-i'm getting a message f-from {}-san!!", addr);

    // this function will need to take in an Arc<Mutex<>> once we know what data we're actually
    // GETTING. if that's a hash map or w/e.
    // we don't need the peer map i'm just confused
}

#[tokio::main]
async fn main() {
    // begin -- warp
    println!("starting...");

    // setting up filters
    let base = warp::path::end()
        .and(warp::fs::file("pages/index.html"));

    let media = warp::path("website").and(warp::fs::dir("website/"));

    // i am an expert in the package i hate
    let routes = warp::get().and(
        base
        .or(media)
        );

    println!("warp filters... ONLINE.\ntarget: TOKIO [LOCKED]\nspawning...");
    // warp::serve returns a future we need to delegate somewhere.
    // we're spawning() a thread here to take that over.
    tokio::spawn(
        warp::serve(routes)
        .run(([127,0,0,1], WARP_PORT)) // was .await
    );

    // now on to tungstenite...
    
    // take our socket
    let try_socket = TcpListener::bind(format!("127.0.0.1:{TUNG_PORT}")).await;

    // or, maybe grab it here
    println!("trying to grab socket {TUNG_PORT}...");
    let listener = try_socket.expect("Failed to bind");


    println!("Listening. to tcp, even: 127.0.0.1:{TUNG_PORT}");
    // sit here serving our tcp like a good boye.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
        // NOTE: we can add a reference to a Arc<Mutex<>> here to pass into the helper function
        // above. we just need to know what sort of data we're actually looking at getting.
    }
}
