use tonic::{transport::Server, Request, Response, Status};

use user::user_service_server::{UserService, UserServiceServer};
use user::{NewUserResponse, NewUserRequest};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Default)]
pub struct NewUserServiceImpl {}

#[tonic::async_trait]
impl UserService for NewUserServiceImpl {
    async fn new_user(
        &self,
        request: Request<NewUserRequest>,
    ) -> Result<Response<NewUserResponse>, Status> {
        let reply = user::NewUserResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = NewUserServiceImpl::default();

    Server::builder()
        .add_service(UserServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}