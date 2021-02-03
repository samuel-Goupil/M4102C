
use std::convert::TryInto;

use libzmq::{prelude::*, ServerBuilder, RadioBuilder, Radio, Group,Msg, TcpAddr};
use structopt::StructOpt;
use radio_chat::{self, Result};
use radio_chat::ContentsMessage;





fn main ()->Result<()>{
    serve();
    Ok(())
    
}

fn handle_request(request:ContentsMessage, radio: &Radio)->Result<()>{

    let mut message=Msg::from(request.payload);
    
    for recipient in request.recipients {
        let group:Group=recipient.try_into()?;
        message.set_group(group);
        radio.send(message.clone())?;
    }
    Ok(())
}

fn serve()->Result<()>{
    let endpoint: TcpAddr=format!("0.0.0.0:{}", radio_chat::RADIO_PORT).try_into()?;
    let server= ServerBuilder::new().bind(endpoint).build()?;

    loop{
        let endpoint: TcpAddr=format!("0.0.0.0:{}", radio_chat::RADIO_PORT).try_into()?;
        let radio = RadioBuilder::new().bind(endpoint).build()?;
        let received_message=server.recv_msg()?;
        println!("message arrivant : {}",received_message.to_str()?);
        let message=serde_json::from_str::<ContentsMessage>(received_message.to_str()?)?;
        handle_request(message,&radio)?;
    }

}

