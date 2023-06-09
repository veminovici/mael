use crate::{egress::Egress, message::Message, payload::Payload};

pub trait Node {
    type Payload: Payload;

    fn from_init(node_id: &str, node_ids: &[String]) -> Self;

    fn handle_message<E: Egress>(
        &mut self,
        msg: Message<Self::Payload>,
        egress: &E,
    ) -> anyhow::Result<()>;
}
