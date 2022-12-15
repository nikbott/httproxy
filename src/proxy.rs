use anyhow::Result;
use httparse::Request;
use log::{error, info, warn};
use std::{
    fs::read,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub fn server(address: String) -> Result<()> {
    println!("Starting server on {}", address);
    info!("Starting server on {}", address);
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        thread::spawn(move || {
            if let Err(e) = handle_connection(&mut stream) {
                println!("Connection error: {}", e);
                error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0; 8192];

    let len = stream.read(&mut buf)?;

    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = Request::new(&mut headers);
    req.parse(&buf[..len])?;

    match req.path {
        Some(path) => {
            if path.contains("monitorando") {
                println!("Access denied to {} -> {}", stream.peer_addr()?, path);
                warn!("Access denied to {} -> {}", stream.peer_addr()?, path);
                let doc = String::from_utf8(read("access_denied.html")?)?;
                let response = format!(
                    "HTTP/1.1 403 Forbidden\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
                    doc.len(),
                    doc
                );
                stream.write(response.as_bytes())?;
            } else {
                let mut resp = reqwest::blocking::get(path)?;
                println!(
                    "{} -> {}; {}",
                    stream.peer_addr()?,
                    resp.remote_addr().unwrap(),
                    resp.status()
                );
                info!(
                    "{} -> {}; {}",
                    stream.peer_addr()?,
                    resp.remote_addr().unwrap(),
                    resp.status()
                );
                resp.copy_to(stream)?;
            }
        }
        None => {}
    }

    Ok(())
}
