use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("localhost:9999").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {
                        break;
                    }
                }
                let mut p = std::path::PathBuf::new();
                p.push("htdocs");
                p.push(resource.trim_start_matches("/"));
                if resource.ends_with("/") {
                    p.push("index.html");
                }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                match File::open(p) {
                    Ok(mut file) => {
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).unwrap();
                        stream.write_all(&buffer).unwrap();
                    }
                    Err(_) => {}
                };
            }
            _ => todo!(),
        }
    }
}
