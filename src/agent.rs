//! Agent implementation — room participants with intention and energy.

/// An agent's contribution to the shared intention field.
#[derive(Debug, Clone)]
pub struct IntentionContribution {
    /// Strength of the intention (0.0 to 1.0).
    pub strength: f64,
    /// Direction of the intention as a 2D vector.
    pub direction: (f64, f64),
    /// Label describing what the agent intends.
    pub label: String,
}

impl IntentionContribution {
    /// Create a new intention contribution.
    pub fn new(strength: f64, direction: (f64, f64), label: impl Into<String>) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            direction,
            label: label.into(),
        }
    }

    /// Neutral intention (no contribution).
    pub fn neutral() -> Self {
        Self {
            strength: 0.0,
            direction: (0.0, 0.0),
            label: "neutral".into(),
        }
    }

    /// Magnitude of the intention vector.
    pub fn magnitude(&self) -> f64 {
        (self.direction.0 * self.direction.0 + self.direction.1 * self.direction.1).sqrt()
    }

    /// Normalize the direction vector.
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.direction.0 /= mag;
            self.direction.1 /= mag;
        }
    }
}

/// An agent participating in rooms.
///
/// Each agent has:
/// - A position in the room's coordinate space
/// - An intention contribution to the shared field
/// - An energy budget governing how long it can act
#[derive(Debug, Clone)]
pub struct RoomAgent {
    /// Unique agent name.
    pub name: String,
    /// Current room the agent is in (None if not in any room).
    pub current_room: Option<String>,
    /// Position in room coordinate space.
    pub position: (f64, f64),
    /// Intention contribution.
    pub intention: IntentionContribution,
    /// Total energy budget.
    pub energy_budget: f64,
    /// Energy used so far.
    pub energy_used: f64,
    /// Whether the agent is active.
    pub active: bool,
}

impl RoomAgent {
    /// Create a new agent with the given name, initially in the specified room.
    pub fn new(name: impl Into<String>, room: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            current_room: Some(room.into()),
            position: (0.5, 0.5),
            intention: IntentionContribution::neutral(),
            energy_budget: 50.0,
            energy_used: 0.0,
            active: true,
        }
    }

    /// Create a detached agent (not in any room).
    pub fn detached(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            current_room: None,
            position: (0.0, 0.0),
            intention: IntentionContribution::neutral(),
            energy_budget: 50.0,
            energy_used: 0.0,
            active: false,
        }
    }

    /// Set a custom energy budget.
    pub fn with_energy_budget(mut self, budget: f64) -> Self {
        self.energy_budget = budget;
        self
    }

    /// Set the agent's position.
    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// Set the agent's intention.
    pub fn with_intention(mut self, intention: IntentionContribution) -> Self {
        self.intention = intention;
        self
    }

    /// Energy remaining.
    pub fn energy_remaining(&self) -> f64 {
        (self.energy_budget - self.energy_used).max(0.0)
    }

    /// Whether the agent has exhausted its energy.
    pub fn is_exhausted(&self) -> bool {
        self.energy_used >= self.energy_budget
    }

    /// Consume energy. Returns false if insufficient.
    pub fn consume_energy(&mut self, amount: f64) -> bool {
        if self.energy_used + amount > self.energy_budget {
            return false;
        }
        self.energy_used += amount;
        true
    }

    /// Move the agent to a new position.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    /// Distance to another agent.
    pub fn distance_to(&self, other: &RoomAgent) -> f64 {
        let dx = self.position.0 - other.position.0;
        let dy = self.position.1 - other.position.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Deactivate the agent.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Reactivate the agent.
    pub fn activate(&mut self) {
        self.active = true;
    }
}
