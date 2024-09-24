use server::Server;

mod handler;
mod router;
mod server;

fn main() {
    // Start  a server
    let server = Server::new("127.0.0.1:3000");
    // Run the server w
    server.run();
}
