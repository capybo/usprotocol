mod response;
mod server;
mod commands;

fn main() {
    let server = server::Server::new(8080, 10);
    server.start();
}
