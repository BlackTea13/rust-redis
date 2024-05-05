use crate::connection::Connection;
use crate::frame::Frame;
use tokio::net::TcpStream;

pub async fn start_repl(socket: TcpStream) {
    let connection = Connection::new(socket);
    loop {}
}
