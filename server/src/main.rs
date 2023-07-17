use dotenv::dotenv;
use tonic::transport::Server;

use self::account::service::Service as AccountService;
use self::pb::account::account_service_server::AccountServiceServer;
use self::user::service::Service as UserService;
use self::pb::user::user_service_server::UserServiceServer;

// Utility modules
mod db;
mod error;

// Domain modules
mod account;
mod user;

pub mod pb {
    tonic::include_proto!("mod");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = db::establish_connection().await?;
    let addr = "127.0.0.1:50000".parse().unwrap();

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(UserServiceServer::new(UserService::new(pool.clone()))))
        .add_service(tonic_web::enable(AccountServiceServer::new(AccountService::new(pool.clone()))))
        .serve(addr)
        .await?;

    println!("gRPC Server listening on {}", addr);

    Ok(())
}
