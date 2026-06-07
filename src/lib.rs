//! # openrooms — Agent-Powered Collaborative Rooms
//!
//! Rooms have topology (room-topology), agents have intention fields
//! (intention-field), and disagreements are decomposed via Hodge theory
//! (hodge-music).
//!
//! ## Core Concepts
//!
//! - **Room**: A collaborative space with connections to other rooms
//! - **RoomAgent**: An agent with position, intention, and energy budget
//! - **Topology**: The connection graph between rooms (doors, warps)
//! - **Session**: Manages agents entering/leaving rooms
//!
//! ## Example
//!
//! ```no_run
//! use openrooms::{Room, RoomAgent, Topology, Session};
//!
//! let mut topology = Topology::new();
//! topology.add_room("lobby");
//! topology.add_room("workshop");
//! topology.connect("lobby", "workshop", openrooms::ConnectionType::Door);
//!
//! let mut session = Session::new(topology);
//! let agent = RoomAgent::new("alice", "lobby");
//! session.admit(agent);
//! ```

mod agent;
mod room;
mod session;
mod topology;

pub use agent::{IntentionContribution, RoomAgent};
pub use room::{Room, RoomState};
pub use session::{Session, SessionError};
pub use topology::{ConnectionType, Door, Topology, TopologyError, Warp};