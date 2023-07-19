fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .include_file("mod.rs")
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                "../proto/account.proto",
                "../proto/collection.proto",
                "../proto/document.proto",
                "../proto/field.proto",
                "../proto/field_group.proto",
                "../proto/user.proto",
            ],
            &["../proto"],
        )
        .unwrap();

        Ok(())
}
