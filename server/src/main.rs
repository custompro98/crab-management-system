use dotenv::dotenv;
use tonic::transport::Server;

use self::user::pb::user_service_server::UserServiceServer;
use self::user::service::Service as UserService;

pub mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "127.0.0.1:50000".parse().unwrap();

    let user_service = UserServiceServer::new(UserService::new());

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(user_service))
        .serve(addr)
        .await?;

    println!("gRPC Server listening on {}", addr);

    Ok(())
}
