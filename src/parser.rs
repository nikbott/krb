use std::net::{ IpAddr, Ipv4Addr };

pub fn get_ip(address: Option<String>) -> IpAddr {
    match address {
        Some(address) => address.parse::<IpAddr>().unwrap(),
        None => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    }
}

pub fn get_mode(mode: Option<String>) -> String {
    match mode {
        Some(mode) => match mode.as_str() {
            "as" => "as".to_string(),
            "ss" => "ss".to_string(),
            "tgs" => "tgs".to_string(),
            _ => "client".to_string(),
        },
            
        None => String::from("client"),
    }
}

pub fn get_port(port: Option<u16>) -> u16 {
    match port {
        Some(port) => port,
        None => 7878,
    }
}

pub fn get_service(service: Option<String>) -> String {
    match service {
        Some(service) => service,
        None => String::from(""),
    }
}

pub fn get_time(time: Option<usize>) -> usize {
    match time {
        Some(time) => time,
        None => 3600,
    }
}

pub fn get_username(username: Option<String>) -> String {
    match username {
        Some(username) => username,
        None => String::from(""),
    }
}
