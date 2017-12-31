extern crate libcontract_utils;

use std::env;
use libcontract_utils::SgxMode;

fn main () {
  // TODO: Improve this.
  let intel_sgx_sdk_dir = match env::var("INTEL_SGX_SDK") {
    Ok(val) => val,
    Err(_) => panic!("Required environment variable INTEL_SGX_SDK not defined")
  };

  let rust_sgx_sdk_dir = match env::var("RUST_SGX_SDK") {
    Ok(val) => val,
    Err(_) => panic!("Required environment variable RUST_SGX_SDK not defined")
  };

  let sgx_mode = match env::var("SGX_MODE") {
    Ok(val) => match val.as_ref() {
      "HW" => SgxMode::Hardware,
      _ => SgxMode::Simulation,
    },
    Err(_) => panic!("Required environment variable SGX_MODE not defined")
  };

  libcontract_utils::build_untrusted(&intel_sgx_sdk_dir, &rust_sgx_sdk_dir, sgx_mode);
}
