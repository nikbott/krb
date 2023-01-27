use std::{fs::File, io::BufReader, net::TcpStream};

use zeroize::Zeroize;

use crate::{comm::*, crypt::*, server_structs::Ss};

pub fn ss_server(mut stream: TcpStream) {
    let file = File::open("ss.json").unwrap();
    let content = BufReader::new(file);
    let as_data: Ss = serde_json::from_reader(content).unwrap();

    recv_line(&stream);

    let response = "SS";
    send_line(&mut stream, response);

    let binding = recv_line(&stream);
    let msg = binding.trim().split(':').collect::<Vec<&str>>();

    let service_token = msg[0];
    let service_ticket = msg[1];

    let service_key = as_data.service.key.unwrap();
    let service_ticket = decrypt(&service_key, service_ticket);

    let split = service_ticket.split(':').collect::<Vec<&str>>();
    let username = split[0];
    let timestamp = split[1];
    let lifetime = split[2];
    let mut skey = split[3].to_string();

    println!(
        "user: {}, timestamp: {}, lifetime: {}, skey: {}",
        username, timestamp, lifetime, skey
    );

    let service_token = decrypt(&skey, service_token);
    println!("service_token: {}", service_token);

    let response = encrypt(
        &skey,
        format!("Hello {}, you have {}s remaining.", username, lifetime).as_str(),
    );
    send_line(&mut stream, response.as_str());

    skey.zeroize();
}
