use futures::prelude::*;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, Multiaddr, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //~ Identities in libp2p are handled using a public/private key pair. Nodes identify each other
    //~ using `PeerId`s, which are generated from the public key.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    //~ A transport in libp2p provides connection-oriented communcation channels, as well as
    //upgrades on top of those, like authentication and encryption.
    //~ The `development_transport` function creates a multiplexed TCP transport with `noise` for
    //~ authenticated encryption
    let transport = libp2p::development_transport(local_key).await?;

    //~ While the transport defines _how_ to send bytes on the network, the `NetworkBehavior`
    //~ defines _what_ to send.
    //~ In the case of the `Ping` example, the `Ping` `NetworkBehavior` sends a ping to a remote and expects to receive
    //~ a pong in return.
    let behavior = Ping::new(PingConfig::new().with_keep_alive(true));

    //~ A `Swarm` drives both a `Transport` and `NetworkBehavior` forward, passing commands from the `NetworkBehavior`
    //~ to the `Transport` as well as events from the `Transport` to the `NetworkBehavior`
    let mut swarm = Swarm::new(transport, behavior, local_peer_id);

    //~ A `Multiaddr` is a self-describing network address and protocol stack that is used to
    //~ establish connections to peers.
    //~ Use a `Multiaddr` to bind the swarm to listen on a socket.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identitied by the multi-address given as the second command-line argument, if
    // any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    //~ You must continously pool the Swarm, allowing it to listen for incomming connections and
    //~ potentially specifying outgoing connections
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}
