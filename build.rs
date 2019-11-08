fn main() {
    tonic_build::compile_protos("proto/filer.proto").unwrap();
    tonic_build::compile_protos("proto/master.proto").unwrap();
    tonic_build::compile_protos("proto/volume.proto").unwrap();
    println!("cargo:rerun-if-changed=proto")
}
