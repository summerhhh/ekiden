extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;

extern crate libcontract_untrusted;
extern crate libcontract_common;

mod generated;
mod server;

use std::env;
use std::thread;

use libcontract_untrusted::enclave;

use generated::compute_web3_grpc::ComputeServer;
use server::ComputeServerImpl;

fn main() {
    let contract_filename = env::args().nth(1).expect("Usage: compute <contract-filename>");

    // Create a new ekiden enclave from the given library.
    let contract = enclave::EkidenEnclave::new(&contract_filename).unwrap();

    // Start the gRPC server.
    let mut server = grpc::ServerBuilder::new_plain();
    let port = 9001;
    server.http.set_port(port);
    server.add_service(ComputeServer::new_service_def(ComputeServerImpl::new(contract)));
    server.http.set_cpu_pool_threads(1);
    let _server = server.build().expect("server");

    println!("Compute node listening at {}", port);

    loop {
        thread::park();
    }
}
