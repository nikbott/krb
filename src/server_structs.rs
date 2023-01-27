use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct As {
    pub version: String,
    pub tgs_key: String,
    pub users: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ss {
    pub version: String,
    pub service: Service,
    pub users: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tgs {
    pub version: String,
    pub tgs_key: String,
    pub services: Vec<Service>,
    pub users: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientJson {
    pub version: String,
    pub services: Vec<Service>,
    pub users: Vec<User>,
}
