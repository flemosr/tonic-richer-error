fn main() {
    prost_build::compile_protos(
        &[
            "proto/google.rpc/status.proto",
            "proto/google.rpc/error_details.proto",
        ],
        &["proto/google.rpc"],
    )
    .unwrap();
}
