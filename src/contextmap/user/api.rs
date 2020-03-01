use tonic::{Request, Status, Response};

use user::user_service_server::{ UserService, UserServiceServer };
use user::{NewUserResponse, NewUserRequest};

mod user {
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

pub fn new() -> UserServiceServer<NewUserServiceImpl> {
    let service = NewUserServiceImpl::default();
    UserServiceServer::new(service)
}
