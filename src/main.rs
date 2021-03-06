use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Alive,
    Dead,
}

type Clock = u64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Peer {
    state: State,
}

impl Peer {
    fn new() -> Self {
        Peer { state: State::Alive }
    }
}

#[derive(Debug)]
struct Registry {
    peers: HashMap<SocketAddr, Peer>,
}

impl Registry {
    fn new() -> Self {
        Registry { peers: HashMap::new() }
    }

    fn add(&mut self, peer: Peer, address: SocketAddr) {
        self.peers.insert(address, peer);
    }
}

struct FailureDetector;

impl FailureDetector {
    fn ping(&self, addr: SocketAddr) {}
}

fn main() {
    let mut registry = Registry::new();
    let peer = Peer::new();
    let addr = "127.0.0.1:8888".parse().unwrap();
    registry.add(peer, addr);
    let detector = FailureDetector;
    detector.ping(addr);
    println!("{:?}", registry);
}
