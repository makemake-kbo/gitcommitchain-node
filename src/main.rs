mod types;
mod mine;

use sled::{
    open,
};

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

}
