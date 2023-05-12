//use std::io::{BufRead, Write};

use mael::{egress::{self, Egress, StdEgress}, ingress::{self, StdIngress}, message::{Message, Body}, payload::Init, pld};
use mael::ingress::Ingress;

pld!(
    enum MyEcho {
        Echo { echo: String },
        EchoOk { echo: String },
    }
);

fn read_init(ingress: &StdIngress) -> anyhow::Result<Message<Init>> {
    let init = ingress.recv().unwrap();
    let msg = serde_json::from_str::<Message<Init>>(&init).unwrap();
    eprintln!("Read msg (INIT): {msg:?}");

    Ok(msg)
}

fn reply_init(egress: &StdEgress, msg: Message<Init>) -> anyhow::Result<()> {
    let reply = Message {
        src: msg.dest.clone(),
        dest: msg.src.clone(),
        body: Body {
            msg_id: msg.body.msg_id.map(|i| i + 1),
            in_reply_to: msg.body.msg_id,
            payload: Init::InitOk,
        }
    };

    let json = serde_json::to_string(&reply).unwrap();
    eprintln!("Sending msg (INIT_OK): {json}");
    egress.send(json).unwrap();

    Ok(())
}

fn read_echo(ingress: &StdIngress) -> anyhow::Result<Message<MyEcho>> {
    let echo = ingress.recv().unwrap();
    let msg = serde_json::from_str::<Message<MyEcho>>(&echo).unwrap();
    eprintln!("Read msg (ECHO): {echo}");

    Ok(msg)
}

fn reply_echo(egress: &StdEgress, msg: &Message<MyEcho>) -> anyhow::Result<()> {
    let MyEcho::Echo { echo } = &msg.body.payload else {
        panic!("Not an echo");
    };

    // Prepare the reply.
    let reply = Message {
        src: msg.dest.clone(),
        dest: msg.src.clone(),
        body: Body {
            msg_id: msg.body.msg_id.map(|i| i + 1),
            in_reply_to: msg.body.msg_id,
            payload: MyEcho::EchoOk { echo: echo.clone() },
        }
    };

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
    let msg = read_init(&ingress).unwrap();
    reply_init(&egress, msg).unwrap();

    // ECHO
    let msg = read_echo(&ingress).unwrap();
    reply_echo(&egress, &msg).unwrap();

    // ECHO
    let msg = read_echo(&ingress).unwrap();
    reply_echo(&egress, &msg).unwrap();
}