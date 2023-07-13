fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "../proto/account.proto",
                "../proto/collection.proto",
                "../proto/document.proto",
                "../proto/field.proto",
                "../proto/fieldgroup.proto",
                "../proto/user.proto",
            ],
            &["../proto"],
        )
        .unwrap();

        Ok(())
}
