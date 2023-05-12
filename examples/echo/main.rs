use mael::{egress::Egress, kernel::StdKernel, message::Message, node::Node, pld};

pld!(
    enum EchoPayload {
        Echo { echo: String },
        EchoOk { echo: String },
    }
);

struct EchoNode;

impl EchoNode {
    pub fn handle_echo<E>(&mut self, egress: &E, msg: Message<EchoPayload>) -> anyhow::Result<()>
    where
        E: Egress,
    {
        match &msg.body.payload {
            EchoPayload::Echo { echo } => {
                let pld = EchoPayload::EchoOk { echo: echo.clone() };
                let reply = msg.into_reply(&pld);

                let json = serde_json::to_string(&reply)?;
                egress.send(json).unwrap();

                Ok(())
            }
            EchoPayload::EchoOk { .. } => Err(anyhow::anyhow!("We cannot handle EchoOk")),
        }
    }
}

impl Node for EchoNode {
    type Payload = EchoPayload;

    fn from_init(_node_id: &str, _node_ids: &[String]) -> Self {
        EchoNode
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
    StdKernel::<EchoNode>::spawn_and_run().unwrap();
}
