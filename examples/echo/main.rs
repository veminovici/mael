use mael::{egress::Egress, kernel::StdKernel, message::Message, node::Node, pld};

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
        match &msg.body.payload {
            MyEcho::Echo { echo } => {
                let pld = MyEcho::EchoOk { echo: echo.clone() };
                let reply = msg.into_reply(&pld);

                let json = serde_json::to_string(&reply)?;
                egress.send(json).unwrap();

                Ok(())
            }
            MyEcho::EchoOk { .. } => Err(anyhow::anyhow!("We cannot handle EchoOk")),
        }
    }
}

impl Node for MyNode {
    type Payload = MyEcho;

    fn from_init(node_id: &str, node_ids: &[String]) -> Self {
        MyNode {
            node_id: node_id.to_string(),
            node_ids: node_ids.to_vec(),
        }
    }

    fn handle_message<E: Egress>(
        &mut self,
        msg: Message<Self::Payload>,
        egress: &E,
    ) -> anyhow::Result<()> {
        self.handle_echo(egress, msg)
    }
}

fn main() {
    eprintln!("Starting Echo ...");
    StdKernel::<MyNode>::spawn_and_run().unwrap();
}
