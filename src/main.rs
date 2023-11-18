mod exec;
mod mine;
mod server;
mod types;

use crate::mine::mine;
use crate::server::accept_request;
use crate::types::Mempool;
use alloy_primitives::address;
use std::sync::Arc;
use std::sync::RwLock;

use sled::open;
use std::net::SocketAddr;

use git2::Repository;

use tokio::net::TcpListener;

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
    let _repo = Repository::init("db").unwrap();

    let mempool = Arc::new(RwLock::new(Mempool::default()));

    let listener = TcpListener::bind("127.0.0.1:3000".parse::<SocketAddr>().unwrap()).await?;
    println!("\x1b[35mInfo:\x1b[0m Bound to: localhost:3000");

    let mempool_miner = Arc::clone(&mempool);

    tokio::task::spawn(async move {
        let _ = mine(
            mempool_miner,
            db_arc,
            address!("4073007498f7188f098902f7BCaF724Fd256Ad82"),
        )
        .await;
    });

    loop {
        let (stream, socketaddr) = listener.accept().await?;
        println!("\x1b[35mInfo:\x1b[0m Connection from: {}", socketaddr);

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        let mempool_clone = Arc::clone(&mempool);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            accept!(io, mempool_clone);
        });
    }
}
