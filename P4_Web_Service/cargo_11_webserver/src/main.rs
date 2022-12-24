use std::{
    net::{TcpListener, TcpStream},
    io::Write, // for strem.write()
    fs,
};

fn main() {
    let listener = match TcpListener::bind("localhost:7878") {
        Ok(x) => x,
        Err(e) => panic!("Error: {}", e),
    };
    for stream in listener.incoming() {
        webcomm(stream.unwrap());
    }
}

fn webcomm(mut stream: TcpStream) {
    let content = fs::read_to_string("html/hello.html").unwrap();
    let length = content.len();
    let prefix = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n");
    let output = format!("{prefix}{content}");
    stream.write(output.as_bytes()).unwrap();
}
