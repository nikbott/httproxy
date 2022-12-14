use anyhow::Result;
use httparse;
use log::{info, error};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn server(address: String) -> Result<()> {
    info!("Starting server on {}", address);
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let stream = stream?;
        info!("New connection: {}", stream.peer_addr()?);

        thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 4096];
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);

    let len = stream.read(&mut buf)?;
    req.parse(&buf[..len])?;

    println!("Request: {:?}", req);

    Ok(())
}
