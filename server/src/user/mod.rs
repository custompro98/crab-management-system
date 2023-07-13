pub mod pb {
    tonic::include_proto!("user");
}

mod repository;
pub mod service;
