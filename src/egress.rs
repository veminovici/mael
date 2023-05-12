use std::io::Write;
use std::sync::mpsc::{channel, Sender};

use anyhow::anyhow;

/// The definition for the egress queue.
pub trait Egress {
    fn send(&self, s: String) -> anyhow::Result<()>;
}

/// Implements an egress queue based on STDOUT.
pub struct StdEgress(Sender<String>);

impl StdEgress {
    /// Spawns a new egress queue instance.
    pub fn spawn() -> Self {
        let (tx, rx) = channel();

        std::thread::spawn(move || {
            let mut stdout = std::io::stdout().lock();

            for msg in rx {
                writeln!(&mut stdout, "{msg}").unwrap();
            }
        });

        Self(tx)
    }
}

impl Egress for StdEgress {
    fn send(&self, s: String) -> anyhow::Result<()> {
        self.0.send(s).map_err(|e| anyhow!(e))
    }
}
