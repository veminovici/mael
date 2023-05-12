use mael::{egress::Egress, kernel::StdKernel, message::Message, node::Node, pld};
use ulid::Generator;

pld!(
    enum UniquePayload {
        Generate,
        GenerateOk { id: String },
    }
);

struct UniqueNode {
    gen: Generator,
    node_id: String,
}

impl UniqueNode {
    pub fn handle_generate<E>(
        &mut self,
        egress: &E,
        msg: Message<UniquePayload>,
    ) -> anyhow::Result<()>
    where
        E: Egress,
    {
        match &msg.body.payload {
            UniquePayload::Generate => {
                let pld = self.generate_pld()?;
                let reply = msg.into_reply(&pld);

                let json = serde_json::to_string(&reply)?;
                egress.send(json).unwrap();

                Ok(())
            }
            UniquePayload::GenerateOk { .. } => Err(anyhow::anyhow!("We cannot handle GenerateOk")),
        }
    }

    fn guid(&mut self) -> anyhow::Result<String> {
        let ulid = self.gen.generate()?.to_string();
        let guid = format!("{}-{}", self.node_id, ulid);
        Ok(guid)
    }

    fn generate_pld(&mut self) -> anyhow::Result<UniquePayload> {
        let id = self.guid()?;
        let pld = UniquePayload::GenerateOk { id };
        Ok(pld)
    }
}

impl Node for UniqueNode {
    type Payload = UniquePayload;

    fn from_init(node_id: &str, _node_ids: &[String]) -> Self {
        UniqueNode {
            gen: Generator::new(),
            node_id: node_id.to_string(),
        }
    }

    fn handle_message<E: Egress>(
        &mut self,
        msg: Message<Self::Payload>,
        egress: &E,
    ) -> anyhow::Result<()> {
        self.handle_generate(egress, msg)
    }
}

fn main() {
    eprintln!("Starting Unique ...");
    StdKernel::<UniqueNode>::spawn_and_run().unwrap();
}
