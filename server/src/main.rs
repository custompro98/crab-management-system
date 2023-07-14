use dotenv::dotenv;
use tonic::transport::Server;

use self::user::pb::user_service_server::UserServiceServer;
use self::user::service::Service as UserService;

pub mod user;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = db::establish_connection().await?;
    let addr = "127.0.0.1:50000".parse().unwrap();

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(UserServiceServer::new(UserService::new(pool))))
        .serve(addr)
        .await?;

    println!("gRPC Server listening on {}", addr);

    Ok(())
}
