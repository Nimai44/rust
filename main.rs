use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, world!")))
}

fn main() {
    println!("Hello, world!");
    //wait for 3 sec
    thread::sleep(Duration::from_secs(3));

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

        let server = Server::bind(&addr).serve(make_svc);

        println!("Listening on http://{}", addr);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });
}