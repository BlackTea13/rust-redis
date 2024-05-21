use goms_mini_project1::server::start_server;
use goms_mini_project1::Result;

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await
}
