//! Topology implementation — room connections, doors, and warps.

use std::collections::HashMap;

/// Type of connection between two rooms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// A standard door — agents can walk through.
    Door,
    /// A warp — instantaneous teleportation between rooms.
    Warp,
    /// A one-way passage.
    OneWay,
}

/// A connection between two rooms.
#[derive(Debug, Clone)]
pub struct Door {
    /// Source room name.
    pub from: String,
    /// Destination room name.
    pub to: String,
    /// Connection type.
    pub connection_type: ConnectionType,
    /// Whether this connection is open.
    pub open: bool,
    /// Energy cost to traverse this connection.
    pub traversal_cost: f64,
}

impl Door {
    /// Create a new door.
    pub fn new(from: impl Into<String>, to: impl Into<String>, ct: ConnectionType) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            connection_type: ct,
            open: true,
            traversal_cost: 0.0,
        }
    }

    /// Set traversal cost.
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.traversal_cost = cost;
        self
    }

    /// Check if an agent can traverse from `from_room` to `to_room`.
    pub fn can_traverse(&self, from_room: &str, to_room: &str) -> bool {
        if !self.open {
            return false;
        }
        match self.connection_type {
            ConnectionType::Door | ConnectionType::Warp => {
                (self.from == from_room && self.to == to_room)
                    || (self.from == to_room && self.to == from_room)
            }
            ConnectionType::OneWay => self.from == from_room && self.to == to_room,
        }
    }
}

/// A warp connection (instantaneous teleportation).
pub type Warp = Door;

/// Errors from topology operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum TopologyError {
    #[error("room not found: {0}")]
    RoomNotFound(String),
    #[error("rooms already connected: {0} ↔ {1}")]
    AlreadyConnected(String, String),
    #[error("connection not found: {0} ↔ {1}")]
    ConnectionNotFound(String, String),
    #[error("connection is closed: {0} ↔ {1}")]
    ConnectionClosed(String, String),
    #[error("circular connection would be created")]
    CircularConnection,
}

/// Room topology — the graph of room connections.
#[derive(Debug, Clone, Default)]
pub struct Topology {
    /// All room names.
    rooms: Vec<String>,
    /// Connections keyed by "from→to".
    connections: HashMap<String, Door>,
}

impl Topology {
    /// Create an empty topology.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a room to the topology.
    pub fn add_room(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.rooms.contains(&name) {
            self.rooms.push(name);
        }
    }

    /// Check if a room exists.
    pub fn has_room(&self, name: &str) -> bool {
        self.rooms.contains(&name.to_string())
    }

    /// Remove a room and all its connections.
    pub fn remove_room(&mut self, name: &str) {
        self.rooms.retain(|r| r != name);
        self.connections.retain(|_, d| d.from != name && d.to != name);
    }

    /// Connect two rooms.
    pub fn connect(
        &mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        ct: ConnectionType,
    ) -> Result<(), TopologyError> {
        let from = from.into();
        let to = to.into();

        if !self.has_room(&from) {
            return Err(TopologyError::RoomNotFound(from));
        }
        if !self.has_room(&to) {
            return Err(TopologyError::RoomNotFound(to));
        }

        let key = connection_key(&from, &to);
        if self.connections.contains_key(&key) {
            return Err(TopologyError::AlreadyConnected(from, to));
        }

        self.connections.insert(key, Door::new(&from, &to, ct));
        Ok(())
    }

    /// Disconnect two rooms.
    pub fn disconnect(&mut self, from: &str, to: &str) -> Result<(), TopologyError> {
        let key = connection_key(from, to);
        if self.connections.remove(&key).is_none() {
            return Err(TopologyError::ConnectionNotFound(from.to_string(), to.to_string()));
        }
        Ok(())
    }

    /// Get all connections from a room.
    pub fn connections_from(&self, room: &str) -> Vec<&Door> {
        self.connections
            .values()
            .filter(|d| d.from == room || (d.connection_type != ConnectionType::OneWay && d.to == room))
            .collect()
    }

    /// Get rooms adjacent to the given room.
    pub fn neighbors(&self, room: &str) -> Vec<&str> {
        let mut neighbors = Vec::new();
        for door in self.connections.values() {
            if door.from == room {
                neighbors.push(door.to.as_str());
            } else if door.connection_type != ConnectionType::OneWay && door.to == room {
                neighbors.push(door.from.as_str());
            }
        }
        neighbors
    }

    /// Check if two rooms are connected.
    pub fn are_connected(&self, a: &str, b: &str) -> bool {
        let key = connection_key(a, b);
        self.connections.contains_key(&key)
    }

    /// Find the shortest path (BFS) between two rooms.
    pub fn shortest_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        if from == to {
            return Some(vec![from.to_string()]);
        }

        let mut visited = vec![from.to_string()];
        let mut queue = vec![(from, vec![from.to_string()])];

        while let Some((current, path)) = queue.pop() {
            for neighbor in self.neighbors(current) {
                if neighbor == to {
                    let mut result = path;
                    result.push(neighbor.to_string());
                    return Some(result);
                }
                if !visited.iter().any(|v| v == neighbor) {
                    visited.push(neighbor.to_string());
                    let mut new_path = path.clone();
                    new_path.push(neighbor.to_string());
                    queue.push((neighbor, new_path));
                }
            }
        }

        None
    }

    /// All room names.
    pub fn rooms(&self) -> &[String] {
        &self.rooms
    }

    /// Number of rooms.
    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    /// Number of connections.
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Open a connection.
    pub fn open_connection(&mut self, a: &str, b: &str) -> Result<(), TopologyError> {
        let key = connection_key(a, b);
        if let Some(door) = self.connections.get_mut(&key) {
            door.open = true;
            Ok(())
        } else {
            Err(TopologyError::ConnectionNotFound(a.into(), b.into()))
        }
    }

    /// Close a connection.
    pub fn close_connection(&mut self, a: &str, b: &str) -> Result<(), TopologyError> {
        let key = connection_key(a, b);
        if let Some(door) = self.connections.get_mut(&key) {
            door.open = false;
            Ok(())
        } else {
            Err(TopologyError::ConnectionNotFound(a.into(), b.into()))
        }
    }
}

/// Create a canonical key for a connection between two rooms.
fn connection_key(a: &str, b: &str) -> String {
    if a <= b {
        format!("{}→{}", a, b)
    } else {
        format!("{}→{}", b, a)
    }
}
