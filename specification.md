# Examples

## ping
Identities in libp2p are handled using a public/private key pair. Nodes identify each other
using `PeerId`s, which are generated from the public key.
A transport in libp2p provides connection-oriented communcation channels, as well as
The `development_transport` function creates a multiplexed TCP transport with `noise` for
authenticated encryption
While the transport defines _how_ to send bytes on the network, the `NetworkBehavior`
defines _what_ to send.
In the case of the `Ping` example, the `Ping` `NetworkBehavior` sends a ping to a remote and expects to receive
a pong in return.
A `Swarm` drives both a `Transport` and `NetworkBehavior` forward, passing commands from the `NetworkBehavior`
to the `Transport` as well as events from the `Transport` to the `NetworkBehavior`
A `Multiaddr` is a self-describing network address and protocol stack that is used to
establish connections to peers.
Use a `Multiaddr` to bind the swarm to listen on a socket.
You must continously pool the Swarm, allowing it to listen for incomming connections and
potentially specifying outgoing connections

