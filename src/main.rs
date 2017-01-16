extern crate noodle;

use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PeerState {
    Alive,
    Suspect,
    Dead,
}

type PeerClock = u64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Peer {
    state: PeerState,
    clock: PeerClock,
}

impl Peer {
    fn new() -> Self {
        Peer {
            state: PeerState::Alive,
            clock: 0,
        }
    }
}

#[derive(Debug)]
struct PeerRegistry {
    peers: HashMap<SocketAddr, Peer>,
}

impl PeerRegistry {
    fn new() -> Self {
        PeerRegistry { peers: HashMap::new() }
    }

    fn add(&mut self, peer: Peer, address: SocketAddr) {
        self.peers.insert(address, peer);
    }
}

fn main() {
    noodle::hello();

    let mut registry = PeerRegistry::new();
    let peer = Peer::new();
    registry.add(peer, "127.0.0.1:8888".parse().unwrap());
    println!("{:?}", registry);
}
