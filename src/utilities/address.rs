use std::net::{IpAddr, TcpListener};

const HOST_ADDRESS: &str = "127.0.0.1:0";

pub struct AppAddress {
    pub listener: TcpListener,
    pub ip: IpAddr,
    pub port: u16,
}

pub fn app_address() -> AppAddress {
    let listener = TcpListener::bind(HOST_ADDRESS).expect("Failed to bind random port");

    let ip = listener.local_addr().unwrap().ip();
    let port = listener.local_addr().unwrap().port();

    AppAddress { listener, ip, port }
}
