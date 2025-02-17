fn main() {
    tonic_build::configure()
        .emit_rerun_if_changed(false)
        .build_server(false)
        .out_dir("src/")
        .compile_protos(&["proto/cache.proto"], &["proto/"])
        .unwrap()
}
