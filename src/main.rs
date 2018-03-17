use std::env;
use std::net::UdpSocket;

struct Config { response: String, port: String }

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = parse_config(&args);
    let addr = format!("{}:{}", "127.0.0.1", config.port);

    let socket = UdpSocket::bind(&addr)
        .expect(&format!("Couldn't bind to {}", addr));
    let mut buf = [0; 10];

    loop {
        let (_number_of_bytes, src_addr) = socket.recv_from(&mut buf)
            .expect("Didn't receive data");
        socket
            .send_to(format!("{}\n", config.response).as_bytes(), &src_addr)
            .expect("Couldn't respond");
    }
}

fn parse_config(args: &[String]) -> Config {
    let response = match args.get(1) {
       Some(m) => m,
       None => "pong",
    };
    let port = match args.get(2) {
       Some(p) => p,
       None => "3000",
    };
    Config {
        response: response.to_owned(),
        port: port.to_owned()
    }
}
