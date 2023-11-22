use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc};
use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

fn main() {
    let server = Server::new(|request, mut response| {
            let mut path = request.uri().path().trim_start_matches("/").split("/");

            match path.next().expect("No path provided") {
                "write" => {
                    let index = path.next().expect("No index provided")
                        .parse::<i32>()
                        .unwrap();
            
                    Ok(response.status(200)
                        .body(fs::read(format!("./db/{}", index)).unwrap())?)
                },
                "read" => {
                    let index = path.next().expect("No index provided")
                        .parse::<i32>()
                        .unwrap();

                    fs::write(
                        format!("./db/{}", index),
                        request.body()
                    ).unwrap();

                    Ok(response.status(200)
                        .body("".as_bytes().to_vec())?)
                },
                _ => {
                    Ok(response.status(404).body("".as_bytes().to_vec())?)
                },
            }
        });

    server.listen("localhost", "9090");
}