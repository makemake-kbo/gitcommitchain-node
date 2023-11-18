use std::convert::Infallible;
use http_body_util::Full;
use hyper::{
    body::Bytes,
    Request,
};

#[macro_export]
macro_rules! accept {
    (
        $io:expr,
    ) => {
        // Bind the incoming connection to our service
        if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
            .serve_connection(
                $io,
                service_fn(|req| {
                    let response = accept_request(
                        req,
                    );
                    response
                }),
            )
            .await
        {
            println!("\x1b[31mErr:\x1b[0m Error serving connection: {:?}", err);
        }
    };
}

pub async fn accept_request(
    tx: Request<hyper::body::Incoming>,
) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    // Send request and measure time
    let response: Result<hyper::Response<Full<Bytes>>, Infallible>;



    response
}

