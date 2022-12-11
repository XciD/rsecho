use bytes::{BufMut, BytesMut};
use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::fmt::Write;
use std::net::SocketAddr;

async fn echo_http(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 1024 * 10 {
        let mut resp = Response::new("Body too big".into());
        *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
        return Ok(resp);
    }

    let mut lines = BytesMut::new();

    // Output method and path
    let _ = lines.write_str(format!("{} request at {}", req.method(), req.uri()).as_str());

    // Output headers
    let _ = lines.write_str("\n\n-- HEADERS --\n");
    for header in req.headers() {
        let _ = lines.write_str(format!("{}: ", header.0).as_str());
        lines.put_slice(header.1.as_bytes());
        lines.put_u8(0x0A);
    }

    // Output body
    let b = req.into_body().data().await;
    if let Some(b) = b {
        let _ = lines.write_str("\n\n-- BODY --\n");
        match b {
            Ok(b) => {
                lines.put_slice(&b);
            }
            Err(e) => {
                let _ = lines.write_str(e.message().to_string().as_str());
            }
        }
    }

    Ok(Response::new(lines.freeze().into()))
}

#[tokio::main]
async fn main() {
    let addr = "[::]:3000".parse::<SocketAddr>().unwrap();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(echo_http))
    });

    println!("Listening on port {}", addr.port());
    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
