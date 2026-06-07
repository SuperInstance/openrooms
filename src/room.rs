//! Room implementation — collaborative spaces with topology and state.

use crate::agent::RoomAgent;
use std::collections::HashMap;

/// Current state of a room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomState {
    /// Room is open and accepting new agents.
    Open,
    /// Room is paused — no state changes allowed.
    Paused,
    /// Room is closed — no agents can enter or act.
    Closed,
}

/// A collaborative room where agents interact.
///
/// Rooms are the fundamental unit of collaboration in openrooms. Each room
/// has a name, a state, a set of agents, and connections to other rooms
/// through the topology system.
#[derive(Debug, Clone)]
pub struct Room {
    /// Unique room name.
    pub name: String,
    /// Current room state.
    pub state: RoomState,
    /// Agents currently in this room, keyed by name.
    pub agents: HashMap<String, RoomAgent>,
    /// Total energy budget for this room.
    pub energy_budget: f64,
    /// Accumulated entropy in this room.
    pub entropy: f64,
    /// Room-level intention field value (aggregated from agents).
    pub intention_field: f64,
    /// Tags/metadata for the room.
    pub tags: Vec<String>,
}

impl Room {
    /// Create a new room with the given name and default state.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: RoomState::Open,
            agents: HashMap::new(),
            energy_budget: 100.0,
            entropy: 0.0,
            intention_field: 0.0,
            tags: Vec::new(),
        }
    }

    /// Create a room with a custom energy budget.
    pub fn with_energy_budget(mut self, budget: f64) -> Self {
        self.energy_budget = budget;
        self
    }

    /// Add a tag to this room.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add an agent to this room. Returns false if the room is closed
    /// or the agent is already present.
    pub fn add_agent(&mut self, agent: RoomAgent) -> bool {
        if self.state == RoomState::Closed {
            return false;
        }
        if self.agents.contains_key(&agent.name) {
            return false;
        }
        self.agents.insert(agent.name.clone(), agent);
        self.recompute_intention_field();
        true
    }

    /// Remove an agent by name. Returns the removed agent, if any.
    pub fn remove_agent(&mut self, name: &str) -> Option<RoomAgent> {
        let agent = self.agents.remove(name);
        if agent.is_some() {
            self.recompute_intention_field();
        }
        agent
    }

    /// Get an agent by name.
    pub fn get_agent(&self, name: &str) -> Option<&RoomAgent> {
        self.agents.get(name)
    }

    /// Get a mutable reference to an agent by name.
    pub fn get_agent_mut(&mut self, name: &str) -> Option<&mut RoomAgent> {
        self.agents.get_mut(name)
    }

    /// Number of agents in the room.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Total energy used by all agents in the room.
    pub fn total_energy_used(&self) -> f64 {
        self.agents.values().map(|a| a.energy_used).sum()
    }

    /// Energy remaining in the room budget.
    pub fn energy_remaining(&self) -> f64 {
        (self.energy_budget - self.total_energy_used()).max(0.0)
    }

    /// Whether the room's energy budget is exhausted.
    pub fn is_energy_exhausted(&self) -> bool {
        self.total_energy_used() >= self.energy_budget
    }

    /// Recompute the room-level intention field from agent contributions.
    fn recompute_intention_field(&mut self) {
        if self.agents.is_empty() {
            self.intention_field = 0.0;
            return;
        }
        let total: f64 = self.agents.values().map(|a| a.intention.strength).sum();
        self.intention_field = total / self.agents.len() as f64;
    }

    /// Tick the room: advance entropy, decay intentions.
    pub fn tick(&mut self) {
        if self.state != RoomState::Open {
            return;
        }
        // Entropy increases proportionally to number of agents
        let entropy_rate = 0.01 * self.agents.len() as f64;
        self.entropy += entropy_rate;

        // Decay agent intentions slightly
        for agent in self.agents.values_mut() {
            agent.intention.strength *= 0.99;
            agent.energy_used += 0.01;
        }
        self.recompute_intention_field();
    }

    /// Compute disagreement measure between agents in this room.
    /// Uses a simplified Hodge decomposition concept: the disagreement
    /// is the variance of intention directions.
    pub fn disagreement(&self) -> f64 {
        if self.agents.len() < 2 {
            return 0.0;
        }
        let agents: Vec<&RoomAgent> = self.agents.values().collect();
        let mut total_disagreement = 0.0;
        let mut count = 0;

        for i in 0..agents.len() {
            for j in (i + 1)..agents.len() {
                let dx = agents[i].intention.direction.0 - agents[j].intention.direction.0;
                let dy = agents[i].intention.direction.1 - agents[j].intention.direction.1;
                total_disagreement += (dx * dx + dy * dy).sqrt();
                count += 1;
            }
        }

        if count == 0 { 0.0 } else { total_disagreement / count as f64 }
    }

    /// Set room state.
    pub fn set_state(&mut self, state: RoomState) {
        self.state = state;
    }

    /// List agent names in the room.
    pub fn agent_names(&self) -> Vec<&str> {
        self.agents.keys().map(|s| s.as_str()).collect()
    }
}
