use std::convert::TryInto;

use libzmq::{prelude::*, DishBuilder, Group, TcpAddr};
use structopt::StructOpt;
use examples::{self, Result};

#[derive(StructOpt)]
struct Options {
    identity: String,
}

fn listen()->Result<()>{
    let options= Options::from_args();
    let endpoint:TcpAddr=format!("{}:{}",options.identity, radio_chat::RADIO_PORT).try_into()?;
    let group: Group="Limoges".try_into()?;
    let dish = DishBuilder::new().connect(endpoint).join(group).build()?;
}