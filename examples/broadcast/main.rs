use std::{
    char::REPLACEMENT_CHARACTER,
    collections::{HashMap, HashSet},
};

use mael::{egress::Egress, kernel::StdKernel, message::Message, node::Node, pld};

pld!(
    enum BroadcastPayload {
        Broadcast {
            message: usize,
        },
        BroadcastOk,
        Read,
        ReadOk {
            messages: HashSet<usize>,
        },
        Topology {
            topology: HashMap<String, Vec<String>>,
        },
        TopologyOk,
    }
);

struct BroadcastNode {
    node_id: String,
    messages: Vec<usize>,
    topology: HashMap<String, Vec<String>>,
}

impl Node for BroadcastNode {
    type Payload = BroadcastPayload;

    fn from_init(node_id: &str, _node_ids: &[String]) -> Self {
        BroadcastNode {
            node_id: node_id.to_string(),
            messages: vec![],
            topology: HashMap::new(),
        }
    }

    fn handle_message<E: Egress>(
        &mut self,
        msg: Message<Self::Payload>,
        egress: &E,
    ) -> anyhow::Result<()> {
        match &msg.body.payload {
            BroadcastPayload::Broadcast { message } => {
                self.messages.push(*message);

                let reply = msg.into_reply(&BroadcastPayload::BroadcastOk);
                let json = serde_json::to_string(&reply)?;
                egress.send(json).unwrap();

                Ok(())
            }
            BroadcastPayload::BroadcastOk => Err(anyhow::anyhow!("We cannot handle BroadcastOk")),
            BroadcastPayload::Read => {
                let mut messages = HashSet::new();
                messages.extend(self.messages.iter());

                let pld = BroadcastPayload::ReadOk { messages };

                let reply = msg.into_reply(&pld);
                let json = serde_json::to_string(&reply)?;
                egress.send(json).unwrap();

                Ok(())
            }
            BroadcastPayload::ReadOk { .. } => Err(anyhow::anyhow!("We cannot handle ReadOk")),
            BroadcastPayload::Topology { topology } => {
                self.topology = topology.clone();

                let reply = msg.into_reply(&BroadcastPayload::TopologyOk);
                let json = serde_json::to_string(&reply)?;
                egress.send(json).unwrap();

                Ok(())
            }
            BroadcastPayload::TopologyOk => Err(anyhow::anyhow!("We cannot handle TopologyOk")),
        }
    }
}

fn main() {
    eprintln!("Starting Broadcast ...");
    StdKernel::<BroadcastNode>::spawn_and_run().unwrap();
}
