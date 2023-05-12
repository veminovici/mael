use serde::{de::DeserializeOwned, Serialize};

use crate::{
    egress::Egress,
    ingress::Ingress,
    message::{Body, Message},
};

pub trait Payload: Clone + std::fmt::Debug + Serialize + DeserializeOwned {}
impl<P: Clone + std::fmt::Debug + Serialize + DeserializeOwned> Payload for P {}

#[macro_export]
macro_rules! pld {
    ($de: ident, $se: ident, $i: item) => {
        use serde::{Deserialize as $de, Serialize as $se};

        #[derive(Debug, Clone, $de, $se)]
        #[serde(tag = "type", rename_all = "snake_case")]
        $i
    };
    ($i: item) => {
        pld!(__DE, __SE, $i);
    };
}

pld!(
    pub enum Init {
        Init {
            node_id: String,
            node_ids: Vec<String>,
        },
        InitOk,
    }
);

pub trait IngressInitExt: Ingress {
    fn read_init_msg(&self) -> anyhow::Result<Message<Init>> {
        let init = self.recv()?;
        let msg = serde_json::from_str::<Message<Init>>(&init)?;
        eprintln!("Read msg (INIT): {msg:?}");

        Ok(msg)
    }
}

impl<I> IngressInitExt for I where I: Ingress {}

pub trait EgressInitExt: Egress {
    fn reply_init(&self, msg: Message<Init>) -> anyhow::Result<()> {
        let reply = msg.into_reply(&Init::InitOk);
        let json = serde_json::to_string(&reply)?;
        self.send(json)
    }

    fn reply_init_msg(&self, msg: Message<Init>) -> anyhow::Result<()> {
        let reply = Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: Body {
                msg_id: msg.body.msg_id.map(|i| i + 1),
                in_reply_to: msg.body.msg_id,
                payload: Init::InitOk,
            },
        };

        let json = serde_json::to_string(&reply)?;
        eprintln!("Sending msg (INIT_OK): {json}");
        self.send(json)?;

        Ok(())
    }
}

impl<E> EgressInitExt for E where E: Egress {}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn se_de_init() {
        let node_id_orig = "nid".to_owned();
        let node_ids_orig = vec!["aid".to_owned(), "bid".to_owned()];

        let pld = Init::Init {
            node_id: node_id_orig.clone(),
            node_ids: node_ids_orig.clone(),
        };
        let json = serde_json::to_string(&pld).unwrap();
        let pld1: Init = serde_json::from_str(&json).unwrap();

        match pld1 {
            Init::Init { node_id, node_ids } => {
                assert_eq!(node_id, node_id_orig);
                assert_eq!(node_ids, node_ids_orig);
            }
            Init::InitOk => assert!(false),
        }
    }

    #[test]
    fn se_de_initok() {
        let pld = Init::InitOk;
        let json = serde_json::to_string(&pld).unwrap();
        let pld1: Init = serde_json::from_str(&json).unwrap();

        match pld1 {
            Init::Init { .. } => assert!(false),
            Init::InitOk => assert!(true),
        }
    }
}
