
use std::convert::TryInto;

use libzmq::{prelude::*, ServerBuilder, RadioBuilder,Client, Radio, Group,Msg, TcpAddr};
use structopt::StructOpt;
use radio_chat::{self, Result};
use radio_chat::ContentsMessage;
use std::io::{self,prelude::*, BufReader};

const NB_ITERATIONS: usize=100;

#[derive(StructOpt)]
struct Options {
    identity: String,
}


fn main (){
    
    
}

fn handle_request(request:ContentsMessage, radio: &Radio)->Result<()>{
    let endpoint: TcpAddr=format!("0.0.0.0:{}", radio_chat::RADIO_PORT).try_into()?;
    let radio = RadioBuilder::new().bind(endpoint).build()?;

    for recipient in request.recipients {
        let mut message=Msg::from(request.payload);
        let group:Group=recipient.try_clone();
        message.set_group(group);
        radio.send(message.clone())?;
    }
    Ok(())
}

fn serve()->Result<()>{
    let endpoint: TcpAddr=format!("0.0.0.0:{}", radio_chat::RADIO_PORT).try_into()?;
    let server= ServerBuilder::new().bind(endpoint).build()?;

    loop{

    }

}