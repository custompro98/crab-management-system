use dotenv::dotenv;
use tonic::transport::Server;

use self::account::handler::Handler as AccountHandler;
use self::pb::account::account_service_server::AccountServiceServer;
use self::field::handler::Handler as FieldHandler;
use self::pb::field::field_service_server::FieldServiceServer;
use self::field_group::handler::Handler as FieldGroupHandler;
use self::pb::field_group::field_group_service_server::FieldGroupServiceServer;
use self::user::handler::Handler as UserHandler;
use self::pb::user::user_service_server::UserServiceServer;

// Utility modules
mod db;
mod error;

// Domain modules
mod account;
mod field;
mod field_group;
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
        .add_service(tonic_web::enable(AccountServiceServer::new(AccountHandler::new(pool.clone()))))
        .add_service(tonic_web::enable(FieldServiceServer::new(FieldHandler::new(pool.clone()))))
        .add_service(tonic_web::enable(FieldGroupServiceServer::new(FieldGroupHandler::new(pool.clone()))))
        .add_service(tonic_web::enable(UserServiceServer::new(UserHandler::new(pool.clone()))))
        .serve(addr)
        .await?;

    println!("gRPC Server listening on {}", addr);

    Ok(())
}
