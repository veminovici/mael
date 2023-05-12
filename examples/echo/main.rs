//use std::io::{BufRead, Write};

use anyhow::bail;
use mael::{
    egress::Egress,
    kernel::{Kernel, StdKernel},
    message::Message,
    node::Node,
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

impl MyNode {
    pub fn handle_echo<E>(&mut self, egress: &E, msg: Message<MyEcho>) -> anyhow::Result<()>
    where
        E: Egress,
    {
        let reply = build_reply_echo(msg).unwrap();
        let json = serde_json::to_string(&reply).unwrap();
        eprintln!("Sending msg (ECHO_OK): {json}");
        egress.send(json).unwrap();

        Ok(())
    }
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
        self.handle_echo(egress, msg)
    }
}

fn build_reply_echo(msg: Message<MyEcho>) -> anyhow::Result<Message<MyEcho>> {
    let MyEcho::Echo { echo } = &msg.body.payload else {
        bail!("Not an echo")
    };

    let pld = MyEcho::EchoOk { echo: echo.clone() };
    let reply = msg.into_reply(&pld);
    Ok(reply)
}

fn main() {
    eprintln!("Starting Echo ...");

    StdKernel::<MyNode>::spawn().run_loop().unwrap();
}
