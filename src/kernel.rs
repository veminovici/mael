use std::marker::PhantomData;

use anyhow::bail;

use crate::{
    egress::StdEgress,
    ingress::StdIngress,
    message::Message,
    node::Node,
    payload::{EgressInitExt, IngressInitExt},
};

pub trait Kernel<N>
where
    N: Node,
{
    fn create_node(&self) -> anyhow::Result<N>;
    fn run_loop(&self) -> anyhow::Result<()>;
}

pub struct StdKernel<N> {
    pub ingress: StdIngress,
    pub egress: StdEgress,
    phantom: PhantomData<N>,
}

impl<N> StdKernel<N>
where
    N: Node,
{
    pub fn spawn() -> Self {
        let ingress = StdIngress::spawn();
        let egress = StdEgress::spawn();

        StdKernel {
            ingress,
            egress,
            phantom: PhantomData,
        }
    }
}

impl<N> Kernel<N> for StdKernel<N>
where
    N: Node,
{
    fn create_node(&self) -> anyhow::Result<N> {
        let msg = self.ingress.read_init_msg().unwrap();
        match &msg.body.payload {
            crate::payload::Init::Init { node_id, node_ids } => {
                let node = N::from_init(node_id.clone(), node_ids.clone());
                self.egress.reply_init_msg(msg)?;
                Ok(node)
            }
            crate::payload::Init::InitOk => bail!("We are expecting an INIT message"),
        }
    }

    fn run_loop(&self) -> anyhow::Result<()> {
        let mut node = self.create_node()?;

        for line in &self.ingress {
            let msg = serde_json::from_str::<Message<N::Payload>>(&line).unwrap();
            node.handle_message(msg, &self.egress)?;
        }

        Ok(())
    }
}
