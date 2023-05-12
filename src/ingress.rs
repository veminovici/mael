use std::{
    io::BufRead,
    sync::mpsc::{channel, Receiver},
};

/// The definition for the ingress queue.
pub trait Ingress {
    fn recv(&self) -> anyhow::Result<String>;
}

/// Implements an ingress queue based on STDIN.
pub struct StdIngress(Receiver<String>);

impl StdIngress {
    /// Spawns a new ingress queue instance.
    pub fn spawn() -> Self {
        let (tx, rx) = channel();

        std::thread::spawn(move || {
            let stdin = std::io::stdin().lock().lines();

            for line in stdin {
                let line = line.unwrap();
                tx.send(line).unwrap();
            }
        });

        Self(rx)
    }

    /// Returns an iterator.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.0.iter(),
        }
    }
}

//
// Ingress implementation
//

impl Ingress for StdIngress {
    #[inline]
    fn recv(&self) -> anyhow::Result<String> {
        self.0.recv().map_err(|e| anyhow::anyhow!(e))
    }
}

//
// IntoIterator implementation
//

impl<'a> IntoIterator for &'a StdIngress {
    type Item = String;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for StdIngress {
    type Item = String;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { rx: self.0 }
    }
}

//
// Iter implementation
//

pub struct Iter<'a> {
    iter: std::sync::mpsc::Iter<'a, String>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(line) if line == "EOI" => None,
            Some(line) => Some(line),
            None => None,
        }
    }
}

//
// IntoIter implementation
//

pub struct IntoIter {
    rx: Receiver<String>,
}

impl Iterator for IntoIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rx.recv() {
            Ok(line) if line == "EOI" => None,
            Ok(line) => Some(line),
            Err(_) => None,
        }
    }
}