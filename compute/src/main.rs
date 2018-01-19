extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;
extern crate thread_local;
extern crate base64;
extern crate reqwest;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate libcontract_common;
extern crate libcontract_untrusted;

mod generated;
mod ias;
mod server;

use std::thread;

use clap::{Arg, App};
use generated::compute_web3_grpc::ComputeServer;
use server::ComputeServerImpl;

fn main() {
    let matches = App::new("Ekiden Compute Node")
                      .version("0.1.0")
                      .author("Jernej Kos <jernej@kos.mx>")
                      .about("Ekident compute node server")
                      .arg(Arg::with_name("contract")
                        .index(1)
                           .value_name("CONTRACT")
                           .help("Signed contract filename")
                           .takes_value(true)
                           .required(true)
                           .display_order(1)
                           .index(1))
                      .arg(Arg::with_name("port")
                           .long("port")
                           .short("p")
                           .takes_value(true)
                           .default_value("9001")
                           .display_order(2))
                      .arg(Arg::with_name("ias-spid")
                           .long("ias-spid")
                           .value_name("SPID")
                           .help("IAS SPID in hex format")
                           .takes_value(true)
                           .required(true))
                      .arg(Arg::with_name("ias-pkcs12")
                           .long("ias-pkcs12")
                           .help("Path to IAS client certificate and private key PKCS#12 archive")
                           .takes_value(true)
                           .required(true))
                      .get_matches();

    let port = value_t!(matches, "port", u16).unwrap_or(9001);

    // Start the gRPC server.
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(
        ComputeServer::new_service_def(
            ComputeServerImpl::new(
                &matches.value_of("contract").unwrap(),
                ias::IASConfiguration {
                    spid: value_t!(matches, "ias-spid", ias::SPID).unwrap_or_else(|e| e.exit()),
                    pkcs12_archive: matches.value_of("ias-pkcs12").unwrap().to_string()
                }
            )
        )
    );
    server.http.set_cpu_pool_threads(1);
    let _server = server.build().expect("server");

    println!("Compute node listening at {}", port);

    loop {
        thread::park();
    }
}
