use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{
    node::Node,
    payload::{Init, Payload},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<P> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: P,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<P> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}

impl<P> Message<P>
where
    P: Payload,
{
    pub fn into_reply(self, payload: &P) -> Self {
        Message {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: Body {
                msg_id: self.body.msg_id.map(|i| i + 1),
                in_reply_to: self.body.msg_id,
                payload: payload.clone(),
            },
        }
    }
}

impl Message<Init> {
    pub fn to_node<N>(&self) -> anyhow::Result<N>
    where
        N: Node,
    {
        let Init::Init { node_id, node_ids } = &self.body.payload else {
            bail!("expected init as the first message");
        };

        let node = N::from_init(node_id.clone(), node_ids.clone());
        Ok(node)
    }
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn reply_withoug_msg_id() {
        let src = "SRC".to_owned();
        let dest = "DEST".to_owned();
        let msg_id = None;
        let in_reply_to = None;
        let payload = 10;
        let reply_payload = 20;

        let msg = Message {
            src: src.clone(),
            dest: dest.clone(),
            body: Body {
                msg_id,
                in_reply_to,
                payload,
            },
        };

        let reply = msg.into_reply(&reply_payload);

        assert_eq!(dest, reply.src);
        assert_eq!(src, reply.dest);
        assert_eq!(msg_id, reply.body.msg_id);
        assert_eq!(in_reply_to, reply.body.in_reply_to);
        assert_eq!(reply_payload, reply.body.payload);
    }

    #[test]
    fn reply_with_msg_id() {
        let src = "SRC".to_owned();
        let dest = "DEST".to_owned();
        let msg_id = 100;
        let in_reply_to = 1000;
        let payload = 10;
        let reply_payload = 20;

        let msg = Message {
            src: src.clone(),
            dest: dest.clone(),
            body: Body {
                msg_id: Some(msg_id),
                in_reply_to: Some(in_reply_to),
                payload,
            },
        };

        let reply = msg.into_reply(&reply_payload);

        assert_eq!(dest, reply.src);
        assert_eq!(src, reply.dest);
        assert_eq!(Some(msg_id + 1), reply.body.msg_id);
        assert_eq!(Some(in_reply_to), reply.body.in_reply_to);
        assert_eq!(reply_payload, reply.body.payload);
    }
}
