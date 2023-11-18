mod mine;
mod types;
mod server;

use sled::open;

use git2::Repository;

use tokio::net::TcpListener;
use tokio::sync::watch;

use hyper::{
    server::conn::http1,
    service::service_fn,
};
use hyper_util_blutgang::rt::TokioIo;

#[tokio::main]
async fn main() {
    // Create/open new sled DB
    let db = open("db").unwrap();

    // Set up our git repo for storing blocks
    //
    // Create/open new folder inside the current path
    // Initialize a git repo if its not there
    let repo = Repository::init("db").unwrap();


    loop {
        let (stream, socketaddr) = listener.accept().await?;
        println!("\x1b[35mInfo:\x1b[0m Connection from: {}", socketaddr);

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            accept!(
                io,
            );
        });
    }
}
