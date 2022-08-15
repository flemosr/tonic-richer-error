fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(
        &["proto/status.proto", "proto/error_details.proto"],
        &["proto/"],
    )?;
    Ok(())
}
