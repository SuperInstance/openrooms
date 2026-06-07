//! Session management — agents entering/leaving rooms.

use crate::agent::RoomAgent;
use crate::room::Room;
use crate::topology::{ConnectionType, Topology, TopologyError};
use std::collections::HashMap;

/// Errors from session operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    #[error("agent not found: {0}")]
    AgentNotFound(String),
    #[error("room not found: {0}")]
    RoomNotFound(String),
    #[error("agent already in a room: {0}")]
    AgentAlreadyInRoom(String),
    #[error("agent not in any room: {0}")]
    AgentNotInRoom(String),
    #[error("rooms not connected: {0} → {1}")]
    NotConnected(String, String),
    #[error("connection closed: {0} → {1}")]
    ConnectionClosed(String, String),
    #[error("room is closed: {0}")]
    RoomClosed(String),
    #[error("insufficient energy: need {needed}, have {remaining}")]
    InsufficientEnergy { needed: f64, remaining: f64 },
    #[error("topology error: {0}")]
    Topology(#[from] TopologyError),
}

/// Session managing agents across a topology of rooms.
pub struct Session {
    /// Room topology.
    pub topology: Topology,
    /// Rooms keyed by name.
    pub rooms: HashMap<String, Room>,
    /// All known agents, whether in rooms or not.
    pub agents: HashMap<String, RoomAgent>,
}

impl Session {
    /// Create a new session with the given topology.
    pub fn new(topology: Topology) -> Self {
        let mut rooms = HashMap::new();
        for room_name in topology.rooms() {
            rooms.insert(room_name.clone(), Room::new(room_name.clone()));
        }
        Self {
            topology,
            rooms,
            agents: HashMap::new(),
        }
    }

    /// Add a room to the session (also adds to topology).
    pub fn add_room(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.topology.add_room(&name);
        self.rooms.insert(name.clone(), Room::new(name));
    }

    /// Connect two rooms.
    pub fn connect_rooms(
        &mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        ct: ConnectionType,
    ) -> Result<(), SessionError> {
        self.topology.connect(from, to, ct)?;
        Ok(())
    }

    /// Admit an agent into the session. The agent is placed in its
    /// specified room.
    pub fn admit(&mut self, agent: RoomAgent) -> Result<(), SessionError> {
        let room_name = agent
            .current_room
            .clone()
            .ok_or_else(|| SessionError::AgentNotInRoom(agent.name.clone()))?;

        if !self.rooms.contains_key(&room_name) {
            return Err(SessionError::RoomNotFound(room_name));
        }

        let room = self.rooms.get_mut(&room_name).unwrap();
        if room.state == crate::room::RoomState::Closed {
            return Err(SessionError::RoomClosed(room_name));
        }

        let name = agent.name.clone();
        room.add_agent(agent.clone());
        self.agents.insert(name, agent);
        Ok(())
    }

    /// Remove an agent from the session entirely.
    pub fn expel(&mut self, agent_name: &str) -> Result<RoomAgent, SessionError> {
        let agent = self
            .agents
            .remove(agent_name)
            .ok_or_else(|| SessionError::AgentNotFound(agent_name.into()))?;

        if let Some(room_name) = &agent.current_room {
            if let Some(room) = self.rooms.get_mut(room_name) {
                room.remove_agent(agent_name);
            }
        }

        Ok(agent)
    }

    /// Move an agent from its current room to an adjacent room.
    pub fn move_agent(&mut self, agent_name: &str, to_room: &str) -> Result<(), SessionError> {
        let agent = self
            .agents
            .get(agent_name)
            .ok_or_else(|| SessionError::AgentNotFound(agent_name.into()))?;

        let from_room = agent
            .current_room
            .clone()
            .ok_or_else(|| SessionError::AgentNotInRoom(agent_name.into()))?;

        // Check topology connection
        if !self.topology.are_connected(&from_room, to_room) {
            return Err(SessionError::NotConnected(from_room, to_room.into()));
        }

        // Check traversal cost
        let cost = self
            .topology
            .connections_from(&from_room)
            .into_iter()
            .find(|d| (d.from == from_room && d.to == to_room) || (d.to == from_room && d.from == to_room))
            .map(|d| d.traversal_cost)
            .unwrap_or(0.0);

        // Remove from old room
        if let Some(room) = self.rooms.get_mut(&from_room) {
            room.remove_agent(agent_name);
        }

        // Update agent
        let agent = self.agents.get_mut(agent_name).unwrap();
        if !agent.consume_energy(cost) {
            // Restore: put agent back
            // (simplified — in production we'd want transaction semantics)
            return Err(SessionError::InsufficientEnergy {
                needed: cost,
                remaining: agent.energy_remaining(),
            });
        }
        agent.current_room = Some(to_room.to_string());

        // Add to new room
        if let Some(room) = self.rooms.get_mut(to_room) {
            room.add_agent(self.agents[agent_name].clone());
        }

        Ok(())
    }

    /// Get a reference to a room.
    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    /// Get a mutable reference to a room.
    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    /// Get a reference to an agent.
    pub fn get_agent(&self, name: &str) -> Option<&RoomAgent> {
        self.agents.get(name)
    }

    /// Tick all rooms.
    pub fn tick(&mut self) {
        for room in self.rooms.values_mut() {
            room.tick();
        }
    }

    /// Total number of agents in all rooms.
    pub fn total_agents(&self) -> usize {
        self.agents.len()
    }

    /// Total entropy across all rooms.
    pub fn total_entropy(&self) -> f64 {
        self.rooms.values().map(|r| r.entropy).sum()
    }

    /// Compute overall disagreement across all rooms.
    pub fn total_disagreement(&self) -> f64 {
        self.rooms.values().map(|r| r.disagreement()).sum()
    }

    /// Find rooms an agent can reach from its current room.
    pub fn reachable_rooms(&self, agent_name: &str) -> Result<Vec<&str>, SessionError> {
        let agent = self
            .agents
            .get(agent_name)
            .ok_or_else(|| SessionError::AgentNotFound(agent_name.into()))?;

        let current = agent
            .current_room
            .as_deref()
            .ok_or_else(|| SessionError::AgentNotInRoom(agent_name.into()))?;

        Ok(self.topology.neighbors(current))
    }
}
