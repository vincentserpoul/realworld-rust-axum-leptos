use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=proto");

    let proto_root = Path::new("proto");
    let default_proto = proto_root.join("service.proto");

    if !default_proto.exists() {
        return;
    }

    tonic_prost_build::configure()
        .compile_protos(&[default_proto.as_path()], &[proto_root])
        .expect("failed to compile gRPC protos");
}
