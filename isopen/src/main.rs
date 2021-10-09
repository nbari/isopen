use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn conn<T: ToSocketAddrs>(addr: T) -> bool {
    let timeout = Duration::from_secs(1);
    let addrs = match addr.to_socket_addrs() {
        Ok(iter) => iter,
        Err(_) => {
            return false;
        }
    };

    for addr in addrs {
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            return true;
        }
    }
    false
}

fn main() {
    let port_range = 80..1024;
    let ip = std::env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    for port in port_range {
        let a = format!("{}:{}", ip, port);
        println!("{} isopen: {}", port, conn(a));
    }
}
