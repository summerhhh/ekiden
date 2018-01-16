extern crate abci;
extern crate futures;
extern crate grpc;
extern crate hyper;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tls_api;
extern crate tokio_core;
extern crate tokio_proto;

mod ekidenmint;
mod errors;
mod tendermint;
mod generated;
mod rpc;
mod state;

//use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use abci::server::{AbciProto, AbciService};
use tokio_proto::TcpServer;

use generated::storage_grpc::StorageServer;
use rpc::StorageServerImpl;
use state::State;

fn main() {
  println!("Ekiden Storage starting... ");
  // Create a shared State object
  let s = Arc::new(Mutex::new(State::new()));

  // Create Tendermint client.
  // We'll use a channel to funnel transactions to Tendermint client
  let tendermint_uri = String::from("http://localhost:46657");
  let (tx, rx) = mpsc::channel();
  let tx = Arc::new(Mutex::new(tx));
  thread::spawn(move || {
    let mut tendermint_client = tendermint::Tendermint::new(tendermint_uri);
    tendermint::proxy_broadcasts(&mut tendermint_client, rx);
  });

  // @todo Remove
  //let (temp_tx, temp_rx) = mpsc::channel();
  //let broadcast_tx = Arc::clone(&tx);
  //thread::spawn(move || {
  //  thread::sleep(Duration::from_secs(3));
  //  broadcast_tx.lock().unwrap().send(tendermint::BroadcastRequest {
  //    chan: temp_tx,
  //    payload: String::from("helloworld").into_bytes(),
  //  });
  //  let result = temp_rx.recv().unwrap();
  //  println!("broadcast output: {:?}", result);
  //});

  // Start the gRPC server.
  let port = 9002;
  let mut rpc_server = grpc::ServerBuilder::new_plain();
  rpc_server.http.set_port(port);
  rpc_server.http.set_cpu_pool_threads(1);
  rpc_server.add_service(StorageServer::new_service_def(StorageServerImpl::new(Arc::clone(&s), Arc::clone(&tx))));
  let _server = rpc_server.build().expect("rpc_server");
  println!("Storage node listening at {}", port);

  // Start the Tendermint ABCI listener
  let abci_listen_addr = "127.0.0.1:46658".parse().unwrap();
  let mut app_server = TcpServer::new(AbciProto, abci_listen_addr);
  app_server.threads(1);
  app_server.serve(move || {
    Ok(AbciService {
      app: Box::new(ekidenmint::Ekidenmint::new(Arc::clone(&s))),
    })
  });

}
