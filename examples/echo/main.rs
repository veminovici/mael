//use std::io::{BufRead, Write};

use mael::ingress::Ingress;
use mael::{
    egress::{self, Egress, StdEgress},
    ingress::{self, StdIngress},
    message::Message,
    payload::{EgressInitExt, IngressInitExt},
    pld,
};

pld!(
    enum MyEcho {
        Echo { echo: String },
        EchoOk { echo: String },
    }
);

fn read_echo(ingress: &StdIngress) -> anyhow::Result<Message<MyEcho>> {
    let echo = ingress.recv().unwrap();
    let msg = serde_json::from_str::<Message<MyEcho>>(&echo).unwrap();
    eprintln!("Read msg (ECHO): {echo}");

    Ok(msg)
}

fn build_reply_echo(msg: Message<MyEcho>) -> anyhow::Result<Message<MyEcho>> {
    let MyEcho::Echo { echo } = &msg.body.payload else {
        panic!("Not an echo");
    };

    let pld = MyEcho::EchoOk { echo: echo.clone() };
    let reply = msg.into_reply(&pld);
    Ok(reply)
}

fn reply_echo(egress: &StdEgress, msg: Message<MyEcho>) -> anyhow::Result<()> {
    let reply = build_reply_echo(msg).unwrap();
    let json = serde_json::to_string(&reply).unwrap();
    eprintln!("Sending msg (ECHO_OK): {json}");
    egress.send(json).unwrap();

    Ok(())
}

fn main() {
    eprintln!("Starting Echo ...");

    let ingress = ingress::StdIngress::spawn();
    let egress = egress::StdEgress::spawn();

    // Init
    let msg = ingress.read_init_msg().unwrap();
    egress.reply_init_msg(msg).unwrap();

    for _i in 0..100 {
        // ECHO
        let msg = read_echo(&ingress).unwrap();
        reply_echo(&egress, msg).unwrap();
    }
}
