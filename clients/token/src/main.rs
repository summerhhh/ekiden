#[macro_use]
extern crate compute_client;

#[macro_use]
extern crate token_api;

#[macro_use]
extern crate clap;

use clap::{Arg, App};

create_client_api!();

fn main() {
    let matches = App::new("Ekiden Token Contract Client")
                      .version("0.1.0")
                      .author("Jernej Kos <jernej@kos.mx>")
                      .about("Client for the Ekiden Token Contract")
                      .arg(Arg::with_name("host")
                           .long("host")
                           .short("h")
                           .takes_value(true)
                           .default_value("localhost")
                           .display_order(1))
                      .arg(Arg::with_name("port")
                           .long("port")
                           .short("p")
                           .takes_value(true)
                           .default_value("9001")
                           .display_order(2))
                      .arg(Arg::with_name("mr-enclave")
                           .long("mr-enclave")
                           .value_name("MRENCLAVE")
                           .help("MRENCLAVE in hex format")
                           .takes_value(true)
                           .required(true)
                           .display_order(3))
                      .get_matches();

    let backend = compute_client::backend::Web3ContractClientBackend::new(
        matches.value_of("host").unwrap(),
        value_t!(matches, "port", u16).unwrap_or(9001)
    ).unwrap();

    let mut client = token::Client::new(
        backend,
        value_t!(matches, "mr-enclave", compute_client::MrEnclave).unwrap_or_else(|e| e.exit())
    ).unwrap();

    // Create new token contract.
    let mut request = token::CreateRequest::new();
    request.set_sender("testaddr".to_string());
    request.set_token_name("Ekiden Token".to_string());
    request.set_token_symbol("EKI".to_string());
    request.set_initial_supply(8);

    let (state, response) = client.create(request).unwrap();

    println!("New state from contract: {:?}", state);
    println!("Response from contract: {:?}", response);

    let (state, response) = client.transfer(state, {
        let mut request = token::TransferRequest::new();
        request.set_sender("testaddr".to_string());
        request.set_destination("b".to_string());
        request.set_value(3);
        request
    }).unwrap();

    println!("New state from contract: {:?}", state);
    println!("Response from contract: {:?}", response);
}
