use server::Server;

mod handler;
mod router;
mod server;

fn main() {
    // Start  a server
    let server = Server::new("127.0.0.1:3005");
    // Run the server w
    server.run();
}
