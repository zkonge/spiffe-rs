fn main() {
    let proto = "proto/workloadapi.proto";

    tonic_build::compile_protos(proto).unwrap();
}
