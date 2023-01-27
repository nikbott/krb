use crate::{parser::*, server::*};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    address: Option<String>,

    #[arg(short, long, default_value = "client")]
    mode: Option<String>,

    #[arg(short, long, default_value = "7878")]
    port: Option<u16>,

    #[arg(short, long)]
    service: Option<String>,

    #[arg(short, long, default_value = "3600")]
    time: Option<usize>,

    #[arg(short, long)]
    username: Option<String>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let address = get_ip(args.address);
    let port = get_port(args.port);
    let mode = get_mode(args.mode);
    let service = get_service(args.service);
    let time = get_time(args.time);
    let username = get_username(args.username);

    match mode.as_str() {
        "as" => {
            println!("Starting AS server on {}:{}", address, port);
            let server = crate::server::Server::new(address, Mode::As, port);
            server.run();
        }
        "ss" => {
            println!("Starting SS server on {}:{}", address, port);
            let server = crate::server::Server::new(address, Mode::Ss, port);
            server.run();
        }
        "tgs" => {
            println!("Starting TGS server on {}:{}", address, port);
            let server = crate::server::Server::new(address, Mode::Tgs, port);
            server.run();
        }
        _ => {
            let client = crate::client::Client::new(
                address, port, service, time, username,
            );
            client.run();
        }
    }

    Ok(())
}
