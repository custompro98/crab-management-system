fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "account.proto",
                "collection.proto",
                "document.proto",
                "field.proto",
                "fieldgroup.proto",
                "user.proto",
            ],
            &["proto"],
        )
        .unwrap();

        Ok(())
}
