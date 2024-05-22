
#[cfg(test)]
mod ping_tests {

    use goms_mini_project1::server::start_server;
    use goms_mini_project1::Frame;
    use goms_mini_project1::connection;

    #[tokio::test]
    async fn test_simple_ping() {
        let address = "127.0.0.1:6379";
        let server = tokio::spawn(async { start_server().await});
        let stream = tokio::net::TcpStream::connect(address).await.expect("Failed to connect to Redis");
        let mut connection = connection::Connection::new(stream);

        let command = "PING".to_string();
        let frame = Frame::Array(vec![Frame::Bulk(command.into())]);

        let _ = connection.write_frame(&frame).await.expect("Failed to send ping command");
        let response = connection.read_frame().await.expect("Failed to get response").unwrap();
        assert_eq!(Frame::Simple("PONG".to_string()), response);
    }


    #[tokio::test]
    async fn test_ping_with_arg() {
        let address = "127.0.0.1:6379";
        let server = tokio::spawn(async { start_server().await});
        let stream = tokio::net::TcpStream::connect(address).await.expect("Failed to connect to Redis");
        let mut connection = connection::Connection::new(stream);

        let command = "PING".to_string();
        let arg = "HelloWorld!".to_string();
        let frame = Frame::Array(vec![Frame::Bulk(command.into()), Frame::Bulk(arg.clone().into())]);

        let _ = connection.write_frame(&frame).await.expect("Failed to send ping command");
        let response = connection.read_frame().await.expect("Failed to get response").unwrap();
        assert_eq!(Frame::Bulk(arg.into()), response);
    }
}
