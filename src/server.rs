use alloy_primitives::Address;
use alloy_primitives::FixedBytes;
use alloy_primitives::U256;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{
    body::Bytes,
    Request,
};
use std::convert::Infallible;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::sync::broadcast;

use serde_json::{
    json,
    Value,
    Value::Null,
};
use simd_json::serde::from_str;

use crate::types::Transaction;

#[macro_export]
macro_rules! accept {
    (
        $io:expr,
        $mempool:expr
    ) => {
        // Bind the incoming connection to our service
        if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
            .serve_connection(
                $io,
                service_fn(|req| {
                    let response = accept_request(req, $mempool.clone());
                    response
                }),
            )
            .await
        {
            println!("\x1b[31mErr:\x1b[0m Error serving connection: {:?}", err);
        }
    };
}

async fn incoming_to_value(tx: Request<Incoming>) -> Result<Value, hyper::Error> {
    #[cfg(feature = "debug-verbose")]
    println!("Incoming request: {:?}", tx);

    let tx = tx.collect().await?.to_bytes().clone();
    let mut tx = from_utf8(&tx).unwrap().to_owned();

    let ret = match unsafe { from_str(&mut tx) } {
        Ok(ret) => ret,
        Err(_) => {
            // Insane error handling
            let ret = json!({
                "id": Null,
                "jsonrpc": "2.0",
                "result": "Invalid JSON",
            });

            return Ok(ret);
        }
    };

    Ok(ret)
}

pub async fn accept_request(
    tx: Request<hyper::body::Incoming>,
    mempool_tx: Arc<broadcast::Sender<Transaction>>,
) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    let tx = incoming_to_value(tx).await.unwrap();

    // Put tx in a Transaction and send it to the mempool
    let tx = Transaction::new(
        tx["origin"].as_str().unwrap().parse::<Address>().unwrap(),
        tx["to"].as_str().unwrap().parse::<Address>().unwrap(),
        tx["value"].as_str().unwrap().parse::<u128>().unwrap(),
        tx["basefee"].as_str().unwrap().parse::<U256>().unwrap(),
        tx["max_basefee"].as_str().unwrap().parse::<U256>().unwrap(),
        tx["max_priority"]
            .as_str()
            .unwrap()
            .parse::<U256>()
            .unwrap(),
        tx["calldata"].as_str().unwrap().as_bytes().to_vec(),
        tx["signature"]
            .as_str()
            .unwrap()
            .parse::<FixedBytes<65>>()
            .unwrap(),
    );

    mempool_tx.send(tx).unwrap();

    // Convert rx to bytes and but it in a Buf
    let body = hyper::body::Bytes::from("ok");

    // Put it in a http_body_util::Full
    let body = Full::new(body);

    //Build the response
    Ok(hyper::Response::builder().status(200).body(body).unwrap())
}
