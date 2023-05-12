use crate::{ingress::StdIngress, egress::StdEgress};

pub trait Kernel {}

pub struct StdKernel {
    pub ingress: StdIngress,
    pub egress: StdEgress,
}

impl StdKernel {
    pub fn spawn() -> Self {
        let ingress = StdIngress::spawn();
        let egress = StdEgress::spawn();

        StdKernel { ingress, egress }
    }
}

impl Kernel for StdKernel {}

