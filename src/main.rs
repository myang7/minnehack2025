use std::{
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

// tungstenite
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

// warp
use warp::Filter;

// ports
const WARP_PORT:u16 = 8000;
const TUNG_PORT:i32 = 8001;

// days are represented as an array of numbers that adds up to 24.
// eg: monday = [8, 2, 3, 1, 5, 5]
// meaning: 8 hours of busy, 2 hours of free, 3 hours of busy, 1 hour of free,
//          5 hours of busy, 5 hours of free.
//
// the bitfield 'busy' defines the starting value: 1 is busy, 0 is free (1 in this case)

struct Schedule { 
    busy: u8, // bitfield representing the days of the week: 0 is not busy 1 is busy
              // i.e. 0 0 1 1 1 0 1 0
             //(unused) S M TuW ThF S
    su:Vec<u8>, // sunday to fit with algea
    m: Vec<u8>,
    tu:Vec<u8>, // chewsday
    w: Vec<u8>, // hump day
    th:Vec<u8>,
    f: Vec<u8>, // the horse's name
    sa:Vec<u8>,
}

struct User {
    name: String,
    friends: Vec<String>,
    schedule: Schedule,
}

// our tcp helper function
async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("o-oh emm gee!! i-i'm getting a message f-from {}-san!!", addr);

    // this function will need to take in an Arc<Mutex<>> once we know what data we're actually
    // GETTING.

    let stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Someone (addr-san?!) fucked up the handshake :(");
    
    let (_, rx) = unbounded();

    let (s_out, s_in) = stream.split();

    let handle_in = s_in.try_for_each(|data| {
        println!("Data: {}", data);

        future::ok(())
    });

    let handle_out = rx.map(Ok).forward(s_out);

    // let handle_out = 
    pin_mut!(handle_in, handle_out);
    future::select(handle_in, handle_out).await;
    // handle_in().await;

    println!("NO!!!! {}-SAN DISCONNECTED </3", addr);
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

    println!("warp filters @ {WARP_PORT}... ONLINE.\ntarget: TOKIO [LOCKED]\nspawning...");
    // warp::serve returns a future we need to delegate somewhere.
    // we're spawning() a thread here to take that over.
    tokio::spawn(
        warp::serve(routes)
        .run(([127,0,0,1], WARP_PORT))
    );

    // now on to tungstenite...
    
    // take our socket
    let try_socket = TcpListener::bind(format!("127.0.0.1:{TUNG_PORT}")).await;

    // or, maybe grab it here
    let listener = try_socket.expect("Failed to bind");

    println!("Listening. to tcp, even: 127.0.0.1:{TUNG_PORT}");
    // sit here serving our tcp like a good boye.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
        // NOTE: we can add a reference to a Arc<Mutex<>> here to pass into the helper function
        // above. we just need to know what sort of data we're actually looking at getting.
    }
}
