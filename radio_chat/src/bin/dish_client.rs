use std::convert::TryInto;

use libzmq::{prelude::*, DishBuilder,Client,  Group, TcpAddr};
use structopt::StructOpt;
use radio_chat::{self, Result};
use radio_chat::ContentsMessage;

#[derive(StructOpt)]
struct Options {
    identity: String,
}




fn main(){

}

fn listen()->Result<()>{
    let options= Options::from_args();
    let endpoint:TcpAddr=format!("{}:{}",options.identity, radio_chat::RADIO_PORT).try_into()?;
    let group: Group="Limoges".try_into()?;
    let dish = DishBuilder::new().connect(endpoint).join(group).build()?;

    loop {
        let message = dish.recv_msg()?;
        println!("{}", message.to_str()?);
    }
}

fn dispatch_line(line: &str, client: &Client)->Result<()>{
    let message:Vec<&str>= line.split(':').collect();
    let recipients:Vec<String>=message[0].split(',').map(|x| x.to_string()).collect();
    let payload=message[1];
    
    let m= ContentsMessage{
        recipients : recipients,
      payload : payload.to_string()
    };

}