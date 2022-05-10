fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/proto")
        .build_client(true)
        .build_server(true)
        .compile(&["proto/snowflake.proto"], &["proto"])?;
    Ok(())
}
