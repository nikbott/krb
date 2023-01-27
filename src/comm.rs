use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn recv_line(stream: &TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line
}

pub fn send_line(stream: &mut TcpStream, line: &str) {
    stream.write(format!("{}\n", line).as_bytes()).unwrap();
    stream.flush().unwrap();
}
