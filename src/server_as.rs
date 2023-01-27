use std::{fs::File, io::BufReader, net::TcpStream, time::SystemTime};

use zeroize::Zeroize;

use crate::{
    comm::*,
    crypt::{encrypt, gen_secret},
    server_structs::As,
};

pub fn as_server(mut stream: TcpStream) {
    let file = File::open("as.json").unwrap();
    let content = BufReader::new(file);
    let as_data: As = serde_json::from_reader(content).unwrap();

    // Initial Handshake
    recv_line(&stream);
    send_line(&mut stream, "AS");

    let auth_request = recv_line(&stream);

    let user = as_data.users[..]
        .into_iter()
        .find(|user| user.username == auth_request[..auth_request.find(':').unwrap()]);

    let lifetime = auth_request[auth_request.find(':').unwrap() + 1..]
        .trim()
        .parse::<usize>()
        .unwrap();

    let user = match user {
        Some(user) => user,
        None => {
            let response = "Invalid username";
            send_line(&mut stream, response);
            return;
        }
    };

    let key = match &user.key {
        Some(key) => key,
        None => {
            let response = "Invalid username";
            send_line(&mut stream, response);
            return;
        }
    };
    let tgs_key = as_data.tgs_key;
    let mut skey = gen_secret();
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let tgt = encrypt(
        tgs_key.as_str(),
        format!("{}:{}:{}:{}", user.username, timestamp, lifetime, skey).as_str(),
    );

    let user_token = encrypt(
        key.as_str(),
        format!("{}:{}:{}", timestamp, lifetime, skey).as_str(),
    );

    send_line(&mut stream, format!("{}:{}", user_token, tgt).as_str());

    skey.zeroize();
}
