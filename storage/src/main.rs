extern crate abci;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;
extern crate tokio_proto;

mod ekidenmint;
mod generated;
mod server;

//use std::env;
use abci::server::{ AbciProto, AbciService };
use tokio_proto::TcpServer;

use generated::storage_grpc::StorageServer;
use server::StorageServerImpl;

fn main() {
  println!("Ekiden Storage starting... ");
  // Start the gRPC server.
  let port = 9002;
  let mut server = grpc::ServerBuilder::new_plain();
  server.http.set_port(port);
  server.http.set_cpu_pool_threads(1);
  server.add_service(StorageServer::new_service_def(StorageServerImpl::new()));
  let _server = server.build().expect("server");
  println!("Storage node listening at {}", port);

  // Start the ABCI listener
  let abci_listen_addr = "127.0.0.1:46658".parse().unwrap();
  let app = ekidenmint::Ekidenmint::new();
  let app_server = TcpServer::new(AbciProto, abci_listen_addr);
  app_server.serve(move || {
    Ok(AbciService {
      app: Box::new(app.clone()),
    })
  });

}
