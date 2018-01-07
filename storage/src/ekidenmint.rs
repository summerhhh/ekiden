// For reference on how to use the ABCI
// https://github.com/tendermint/abci
// https://github.com/tendermint/basecoin/
use std::sync::{Arc, Mutex};
use abci::application::Application;
use abci::types;

use state::State;

//#[derive(Copy, Clone)]
#[derive(Clone)]
pub struct Ekidenmint {
  name: String,
  state: Arc<Mutex<State>>,
}

impl Ekidenmint {
  pub fn new(state: Arc<Mutex<State>>) -> Ekidenmint {
    Ekidenmint{
      name: String::from("test"),
      state: state,
    }
  }
}

impl Application for Ekidenmint {
  fn info(&self, req: &types::RequestInfo) -> types::ResponseInfo {
    // @todo
    println!("info");
    types::ResponseInfo::new()
  }

  fn set_option(&self, req: &types::RequestSetOption) -> types::ResponseSetOption {
    // @todo
    println!("set_option {}:{}", req.get_key(), req.get_value());
    types::ResponseSetOption::new()
  }

  fn query(&self, p: &types::RequestQuery) -> types::ResponseQuery {
    // @todo
    println!("query");
    types::ResponseQuery::new()
  }

  fn check_tx(&self, p: &types::RequestCheckTx) -> types::ResponseCheckTx {
    //println!("check_tx");
    let mut resp = types::ResponseCheckTx::new();
    match State::check_tx(p.get_tx()) {
      Ok(_) => {
	resp.set_code(types::CodeType::OK);
      },
      Err(error) => {
	resp.set_code(types::CodeType::BaseEncodingError);
	resp.set_log(error);
      },
    }
    return resp;
  }

  fn init_chain(&self, _p: &types::RequestInitChain) -> types::ResponseInitChain {
    // Plugin support in https://github.com/tendermint/basecoin/blob/master/app/app.go
    //println!("init_chain");
    types::ResponseInitChain::new()
  }

  fn begin_block(&self, _p: &types::RequestBeginBlock) -> types::ResponseBeginBlock {
    // Plugin support in https://github.com/tendermint/basecoin/blob/master/app/app.go
    //println!("begin_block");
    types::ResponseBeginBlock::new()
  }

  fn deliver_tx(&self, p: &types::RequestDeliverTx) -> types::ResponseDeliverTx {
    //println!("deliver_tx");
    let mut resp = types::ResponseDeliverTx::new();
    let tx = p.get_tx();
    match State::check_tx(tx) {
      Ok(_) => {
      	// Set the state
	let mut s = self.state.lock().unwrap();
	s.set_latest(tx.to_vec());
	// Respond
	resp.set_code(types::CodeType::OK);
      },
      Err(error) => {
	resp.set_code(types::CodeType::BaseEncodingError);
	resp.set_log(error);
      },
    }
    return resp;
  }

  fn end_block(&self, _p: &types::RequestEndBlock) -> types::ResponseEndBlock {
    // Plugin support in https://github.com/tendermint/basecoin/blob/master/app/app.go
    //println!("end_block");
    types::ResponseEndBlock::new()
  }

  fn commit(&self, p: &types::RequestCommit) -> types::ResponseCommit {
    // @todo
    println!("commit");
    types::ResponseCommit::new()
  }

  fn echo(&self, p: &types::RequestEcho) -> types::ResponseEcho {
    let mut response = types::ResponseEcho::new();
    response.set_message(p.get_message().to_owned());
    return response;
  }

  fn flush(&self, p: &types::RequestFlush) -> types::ResponseFlush {
    // Appears to be unused in https://github.com/tendermint/basecoin/blob/master/app/app.go
    //println!("flush");
    types::ResponseFlush::new()
  }

}
