use std::{fs::File, io::BufReader, net::TcpStream};

use zeroize::Zeroize;

use crate::{comm::*, crypt::*, server_structs::Tgs};

pub fn tgs_server(mut stream: TcpStream) {
    let file = File::open("tgs.json").unwrap();
    let content = BufReader::new(file);
    let tgs_data: Tgs = serde_json::from_reader(content).unwrap();

    let tgs_key = tgs_data.tgs_key;

    recv_line(&stream);
    send_line(&mut stream, "TGS");

    let binding = recv_line(&stream);
    let msg = binding.trim().split(':').collect::<Vec<&str>>();
    let user_token = msg[0];
    let tgt = msg[1];
    println!("user_token: {}", &user_token);
    println!("tgt: {}", &tgt);

    let tgt = decrypt(&tgs_key, &tgt);
    println!("tgt: {}", tgt);

    let mut skey = tgt[tgt.rfind(":").unwrap() + 1..].trim().to_string();
    let user_token = decrypt(&skey, &user_token);
    println!("user_token: {}", user_token);

    let slice = user_token.split(':').collect::<Vec<&str>>();
    let username = slice[0];
    let service_name = slice[1];
    let timestamp = slice[2];
    let lifetime = slice[3];

    let service_key = tgs_data
        .services
        .into_iter()
        .find(|service| service.name == service_name)
        .unwrap()
        .key
        .unwrap();
    
    let mut service_skey = gen_secret();
    let service_ticket = format!("{}:{}:{}:{}", username, timestamp, lifetime, service_skey);
    let service_token = format!("{}:{}:{}", service_skey, timestamp, lifetime);
    println!("service_ticket: {}, service_token: {}", service_ticket, service_token);
    let service_ticket = encrypt(&service_key, &service_ticket);
    let service_token = encrypt(&skey, &service_token);
    println!("service_ticket: {}, service_token: {}", service_ticket, service_token);

    send_line(&mut stream, format!("{}:{}", service_token, service_ticket).as_str());

    skey.zeroize();
    service_skey.zeroize();
}
