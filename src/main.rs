mod mine;
mod types;
mod server;
mod exec;

use std::sync::Arc;
use crate::{
    types::Transaction,
    server::accept_request,
};

use std::net::SocketAddr;
use sled::open;

use git2::Repository;

use tokio::net::TcpListener;
use tokio::sync::broadcast;

use hyper::{
    server::conn::http1,
    service::service_fn,
};
use hyper_util_blutgang::rt::TokioIo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create/open new sled DB
    let db = open("db").unwrap();
    let db_arc = Arc::new(db);

    // Set up our git repo for storing blocks
    //
    // Create/open new folder inside the current path
    // Initialize a git repo if its not there
    let repo = Repository::init("db").unwrap();


    // create mempool channel
    let (mempool_tx, mempool_rx) = broadcast::channel::<Transaction>(1024);
    let mempool_tx_arc = Arc::new(mempool_tx);

    let listener = TcpListener::bind("127.0.0.1:3000".parse::<SocketAddr>().unwrap()).await?;
    println!("\x1b[35mInfo:\x1b[0m Bound to: localhost:3000");

    loop {
        let (stream, socketaddr) = listener.accept().await?;
        println!("\x1b[35mInfo:\x1b[0m Connection from: {}", socketaddr);

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        let mempool_clone = Arc::clone(&mempool_tx_arc);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            accept!(
                io,
                mempool_clone
            );
        });
    }
}
