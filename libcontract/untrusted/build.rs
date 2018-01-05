extern crate protoc_rust;

use std::fs::File;
use std::io::Write;

fn main() {
    // Compile .proto files
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/generated/",
        input: &["src/common/enclave_rpc.proto"],
        includes: &["src/common/"],
    }).expect("protoc");

    // Generate a mod.rs for all generated modules
    let mut file = File::create("./src/generated/mod.rs").unwrap();
    file.write_all(b"
        pub mod enclave_rpc;
    ").unwrap();
}