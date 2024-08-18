fn main() {
    let config = tonic_build::configure().disable_package_emission();

    #[cfg(feature = "bytes")]
    let config = config.bytes(&["."]);

    config
        .compile(&["proto/workloadapi.proto"], &["proto"])
        .unwrap();
}
