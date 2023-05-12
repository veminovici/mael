//use std::io::{BufRead, Write};

use mael::{
    egress::{self, Egress, StdEgress},
    ingress,
    kernel::{Kernel, StdKernel},
    message::Message,
    node::Node,
    payload::{EgressInitExt, IngressInitExt},
    pld,
};

pld!(
    enum MyEcho {
        Echo { echo: String },
        EchoOk { echo: String },
    }
);

struct MyNode {
    node_id: String,
    node_ids: Vec<String>,
}

impl Node for MyNode {
    type Payload = MyEcho;

    fn from_init(node_id: String, node_ids: Vec<String>) -> Self {
        MyNode { node_id, node_ids }
    }

    fn handle_message<E: Egress>(
        &mut self,
        msg: Message<Self::Payload>,
        egress: &E,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

fn read_echo(line: String) -> anyhow::Result<Message<MyEcho>> {
    let msg = serde_json::from_str::<Message<MyEcho>>(&line).unwrap();
    eprintln!("Read msg (ECHO): {line}");

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

    let kernel = StdKernel::<MyNode>::spawn();

    // Create the node
    let node = kernel.create_node().unwrap();

    for line in &kernel.ingress {
        let msg = read_echo(line).unwrap();
        reply_echo(&kernel.egress, msg).unwrap();
    }
}
