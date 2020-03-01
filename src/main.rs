use tonic::transport::Server;

mod contextmap;

use contextmap::user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    Server::builder()
        .add_service(user::api::new())
        .serve(addr)
        .await?;

    Ok(())
}