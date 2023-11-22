use std::{collections::HashMap, io::{self, Read}, fs::File, time::Duration};
use astra::{Body, ResponseBuilder, Server, Request, Response, ConnectionInfo};

fn main() -> io::Result<()> {
    let mut memory: HashMap<i32, String> = HashMap::new();

    Server::bind("localhost:3000")
        .serve(move |req: Request, _info: ConnectionInfo| {
            // Putting the worker thread to sleep will allow
            // other workers to run.
            std::thread::sleep(Duration::from_secs(1));

            let index = req.uri()..headers()["index"]
                .to_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let body = Body::wrap_reader(
                File::open(
                    format!("./db/{}",
                        memory
                        .get(&index)
                        .unwrap()
                    )
                ).unwrap());
    
            ResponseBuilder::new()
                .header("Content-Type", "text/html")
                .body(body)
                .unwrap()
        })?;

    Ok(())
}