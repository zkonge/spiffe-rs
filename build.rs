fn main() {
    let proto = "proto/workloadapi.proto";

    tonic_build::configure()
        .disable_package_emission()
        .compile(&[proto], &["proto"])
        .unwrap();
}
