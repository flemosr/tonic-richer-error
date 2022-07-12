fn main() {
    tonic_build::compile_protos("proto/schedule/schedule.proto").unwrap();
}
