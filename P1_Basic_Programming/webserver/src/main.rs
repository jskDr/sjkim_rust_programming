use std::{
    net::{TcpListener,TcpStream},
    io::Write, // for strem.write() 
    fs,
};

fn main() {
    let listener = TcpListener::bind("localhost:6789").unwrap();
    for stream in listener.incoming() {
        webcomm(stream.unwrap());
    }
}

fn webcomm(mut stream: TcpStream) {
    let contents = fs::read_to_string("html/hello.html").unwrap();
    let length = contents.len();
    let prefix = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n");
    let response = format!(
        "{prefix}{contents}");
    stream.write(response.as_bytes()).unwrap();
}