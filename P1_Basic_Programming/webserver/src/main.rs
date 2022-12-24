use std::{
    net::{TcpListener, TcpStream},
    io::Write, // for strem.write()
    fs,
};

fn main() {
    let listener = TcpListener::bind("localhost:7777").unwrap();
}
