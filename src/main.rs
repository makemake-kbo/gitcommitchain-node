mod mine;
mod types;
mod server;

use sled::open;

use git2::Repository;

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

        // Clone the shared `rpc_list_rwlock` and cache for use in the closure
        let rpc_list_rwlock_clone = Arc::clone(&rpc_list_rwlock);
        let cache_clone = Arc::clone(&cache);
        let head_cache_clone = Arc::clone(&head_cache);
        let finalized_rx_clone = Arc::clone(&finalized_rx_arc);
        let named_blocknumbers_clone = Arc::clone(&named_blocknumbers);
        let config_clone = Arc::clone(&config);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            accept!(
                io,
                &rpc_list_rwlock_clone,
                &cache_clone,
                &finalized_rx_clone,
                &named_blocknumbers_clone,
                &head_cache_clone,
                &config_clone
            );
        });
    }

}
