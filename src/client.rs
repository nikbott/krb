use sha2::{Digest, Sha256};
use std::{
    io::Write,
    net::{IpAddr, SocketAddr, TcpStream},
};
use zeroize::Zeroize;

use crate::{comm::*, crypt::*};

pub struct Client {
    address: IpAddr,
    port: u16,
    service: String,
    time: usize,
    username: String,
}

impl Client {
    pub fn new(
        address: IpAddr,
        port: u16,
        service: String,
        time: usize,
        username: String,
    ) -> Client {
        Client {
            address,
            port,
            service,
            time,
            username,
        }
    }

    pub fn run(&self) {
        let socket = SocketAddr::new(self.address, self.port);
        let mut stream = TcpStream::connect(socket).unwrap();

        let line = "kinit";
        send_line(&mut stream, line);

        match recv_line(&stream).as_str().trim_end() {
            "AS" => self.handle_as(&mut stream),
            "SS" => self.handle_ss(&mut stream),
            "TGS" => self.handle_tgs(&mut stream),
            _ => panic!("Invalid response"),
        }
    }

    fn handle_as(&self, stream: &mut TcpStream) {
        let user = match self.username.as_str() {
            "" => {
                let mut user = String::new();
                print!("username: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut user).unwrap();
                user.trim_end().to_string()
            }
            _ => self.username.clone(),
        };

        send_line(stream, format!("{}:{}", user, self.time).as_str());
        let response = recv_line(stream);

        let user_token = response[..response.find(':').unwrap()].to_string();
        // hash password and salt with sha256 and encode as hex
        let mut buf = [0u8; 64];
        let hash = Sha256::digest(format!(
            "{}{}",
            rpassword::prompt_password("password: ").unwrap(),
            user
        ));
        let hex_hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();
        let user_token = decrypt(&hex_hash, &user_token);

        let tgt = response[response.find(':').unwrap() + 1..].to_string();

        std::fs::write("tgt", tgt.trim().as_bytes()).unwrap();
        std::fs::write("user_token", user_token.trim().as_bytes()).unwrap();
    }

    fn handle_ss(&self, stream: &mut TcpStream) {
        let service_token = std::fs::read_to_string("service_token").unwrap();
        let service_ticket = std::fs::read_to_string("service_ticket").unwrap();

        let user = match self.username.as_str() {
            "" => {
                let mut user = String::new();
                print!("username: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut user).unwrap();
                user.trim_end().to_string()
            }
            _ => self.username.clone(),
        };

        let service = match self.service.as_str() {
            "" => {
                let mut service = String::new();
                print!("service: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut service).unwrap();
                service.trim_end().to_string()
            }
            _ => self.service.clone(),
        };

        let split = service_token.split(':').collect::<Vec<&str>>();
        let mut skey = split[0].to_string();
        let timestamp = split[1];
        let lifetime = split[2];

        let token = format!("{}:{}:{}:{}", user, service, timestamp, lifetime);
        let token = encrypt(&skey, &token);

        send_line(stream, format!("{}:{}", token, service_ticket).as_str());
        let response = recv_line(stream);

        let response = decrypt(&skey, &response);
        println!("{}", response);

        skey.zeroize();
    }

    fn handle_tgs(&self, stream: &mut TcpStream) {
        let tgt = std::fs::read_to_string("tgt").unwrap();
        let user_token = std::fs::read_to_string("user_token").unwrap();

        let user = match self.username.as_str() {
            "" => {
                let mut user = String::new();
                print!("username: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut user).unwrap();
                user.trim_end().to_string()
            }
            _ => self.username.clone(),
        };

        let service = match self.service.as_str() {
            "" => {
                let mut service = String::new();
                print!("service: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut service).unwrap();
                service.trim_end().to_string()
            }
            _ => self.service.clone(),
        };

        let timestamp = user_token[..user_token.find(':').unwrap()].to_string();
        let lifetime = user_token
            [user_token.find(':').unwrap() + 1..user_token.rfind(':').unwrap()]
            .parse::<usize>()
            .unwrap();
        let mut skey = user_token[user_token.rfind(':').unwrap() + 1..].to_string();

        let token = encrypt(
            skey.as_str(),
            format!("{}:{}:{}:{}", user, service, timestamp, lifetime).as_str(),
        );

        send_line(stream, format!("{}:{}", token, tgt).as_str());
        let response = recv_line(stream);

        let service_token = response[..response.find(':').unwrap()].to_string();
        let service_ticket = response[response.find(':').unwrap() + 1..].to_string();
        let service_token = decrypt(&skey, service_token.as_str());

        std::fs::write("service_token", service_token.trim().as_bytes()).unwrap();
        std::fs::write("service_ticket", service_ticket.trim().as_bytes()).unwrap();

        skey.zeroize();
    }
}
