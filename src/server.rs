#[macro_export]
macro_rules! accept {
    (
        $io:expr,
        $rpc_list_rwlock:expr,
        $cache:expr,
        $finalized_rx:expr,
        $named_numbers:expr,
        $head_cache:expr,
        $config:expr
    ) => {
        // Bind the incoming connection to our service
        if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
            .serve_connection(
                $io,
                service_fn(|req| {
                    let response = accept_request(
                        req,
                        Arc::clone($rpc_list_rwlock),
                        $finalized_rx,
                        $named_numbers,
                        $head_cache,
                        Arc::clone($cache),
                        $config,
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
