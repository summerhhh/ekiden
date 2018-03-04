extern crate ekiden_tools;
extern crate protoc_rust;

fn main() {
    ekiden_tools::generate_mod("src/generated", &["enclave_rpc", "enclave_services"]);

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/generated/",
        input: &["src/enclave_rpc.proto", "src/enclave_services.proto"],
        includes: &["src/"],
    }).expect("protoc");
}