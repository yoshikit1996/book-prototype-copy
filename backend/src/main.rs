use std::collections::HashMap;
use tonic::transport::Server;
use config::{Config, File};

mod contextmap;
use contextmap::user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut config_source = Config::default();
    config_source.merge(File::with_name("src/conf/http.json")).unwrap();
    let conf = config_source.try_into::<HashMap<String, String>>().unwrap();

    let host = conf.get("host").unwrap();
    let port = conf.get("port").unwrap();
    let ip_address = format!("{}:{}", host, port);

    Server::builder()
        .add_service(user::api::new())
        .serve(ip_address.parse().unwrap())
        .await?;

    Ok(())
}