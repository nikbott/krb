use std::{
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    thread,
};

use crate::{server_as::*, server_ss::*, server_tgs::*};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    As,
    Ss,
    Tgs,
}

#[derive(Clone, Copy)]
pub struct Server {
    address: IpAddr,
    mode: Mode,
    port: u16,
}

impl Server {
    fn handle_client(self, stream: TcpStream) {
        match self.mode {
            Mode::As => as_server(stream),
            Mode::Ss => ss_server(stream),
            Mode::Tgs => tgs_server(stream),
        }
    }

    pub fn new(address: IpAddr, mode: Mode, port: u16) -> Server {
        Server {
            address,
            mode,
            port,
        }
    }

    pub fn run(self) {
        let socket = SocketAddr::new(self.address, self.port);
        let listener = TcpListener::bind(socket).unwrap();

        // println!("Running {:?} server on {}:{}", self.mode, self.address, self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        self.handle_client(stream);
                    });
                }
                Err(e) => { println!("Error: {}", e); }
            }
        }
    }
}
