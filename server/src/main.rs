use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration};
use astra::{Body, ResponseBuilder, Server, Request, Response, ConnectionInfo};

fn main() -> io::Result<()> {
    let mut memory: Vec<i32> = vec![];

    Server::bind("localhost:3000").serve(move |mut req: Request, _info| {
            // Putting the worker thread to sleep will allow
            // other workers to run.
            std::thread::sleep(Duration::from_secs(1));

            let mut path = req.uri().path().trim_start_matches("/").split("/");

            match path.next().expect("No path provided") {
                "write" => {
                    let index = path.next().expect("No index provided")
                        .parse::<i32>()
                        .unwrap();
        
                    let body = Body::wrap_reader(
                        File::open(format!("./db/{}", index)).unwrap());
            
                    ResponseBuilder::new()
                        .status(200)
                        .body(body)
                        .unwrap()
                },
                "read" => {
                    let index = path.next().expect("No index provided")
                        .parse::<i32>()
                        .unwrap();

                    let buf = &mut vec![];

                    req.body_mut().reader().read_to_end(buf).unwrap();

                    fs::write(
                        format!("./db/{}", index),
                        buf
                    ).unwrap();

                    memory.push(index);

                    ResponseBuilder::new()
                        .status(200)
                        .body(Body::empty())
                        .unwrap()
                },
                _ => {
                    ResponseBuilder::new()
                        .status(404)
                        .body(Body::empty())
                        .unwrap()
                },
            }


        })?;

    Ok(())
}