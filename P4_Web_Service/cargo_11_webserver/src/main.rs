use std::{
    net::{TcpListener, TcpStream},
    io::Write, // for strem.write()
    fs,
};

fn main() {
    let listener = match TcpListener::bind("localhost:7878").unwrap();

}
