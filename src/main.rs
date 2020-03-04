use tonic::transport::Server;
use hocon::HoconLoader;

mod contextmap;

use contextmap::user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = HoconLoader::new()
        .load_file("src/conf/http.conf")?
        .hocon()?;

    let host = conf["host"].as_string().unwrap();
    let port = conf["port"].as_string().unwrap();
    let ip_address = format!("{}:{}", host, port);

    Server::builder()
        .add_service(user::api::new())
        .serve(ip_address.parse().unwrap())
        .await?;

    Ok(())
}