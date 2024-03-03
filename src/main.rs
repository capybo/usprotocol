mod response;
mod server;
mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    let port: Result<u16, _> = args[1].parse();

    if let (Ok(port)) = port {
        let server = server::Server::new(port, 10);
        server.start();
    }
}
