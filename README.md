# openrooms

**Agent-powered collaborative rooms with topology, intention fields, and Hodge decomposition.**

## Concept

In openrooms, agents don't just chat — they inhabit rooms with physical topology. Rooms connect through doors and warps. Agents carry intention fields that interact with each other. Disagreements between agents are decomposed using Hodge theory concepts.

This is **agents-as-rooms**: the collaborative space IS the application.

## Architecture

```
┌──────────┐    Door    ┌──────────┐    Warp    ┌──────────┐
│  Lobby   │────────────│ Workshop │════════════│  Garden  │
│          │            │          │            │          │
│ ● alice  │            │ ● carol  │            │ ● eve    │
│ ● bob    │            │          │            │          │
└──────────┘            └──────────┘            └──────────┘
    ↕                       ↕                       ↕
 intention              intention               intention
  field                   field                   field
    ↕                       ↕                       ↕
  energy                 energy                  energy
 budget                  budget                  budget
```

## Core Types

- **`Room`** — A collaborative space with agents, energy budget, and entropy
- **`RoomAgent`** — An agent with position, intention contribution, and energy
- **`Topology`** — Connection graph between rooms (doors, warps, one-way passages)
- **`Session`** — Manages agent lifecycle: admit, move, expel

## Quick Start

```rust
use openrooms::*;

// Create a topology with rooms
let mut topo = Topology::new();
topo.add_room("lobby");
topo.add_room("workshop");
topo.add_room("garden");
topo.connect("lobby", "workshop", ConnectionType::Door).unwrap();
topo.connect("lobby", "garden", ConnectionType::Warp).unwrap();

// Create a session
let mut session = Session::new(topo);

// Admit agents
let alice = RoomAgent::new("alice", "lobby")
    .with_energy_budget(100.0)
    .with_intention(IntentionContribution::new(0.8, (1.0, 0.0), "explore"));
session.admit(alice).unwrap();

// Move agents between rooms
session.move_agent("alice", "workshop").unwrap();

// Tick the simulation
session.tick();
println!("Total entropy: {}", session.total_entropy());
```

## Intention Fields

Each agent contributes an intention to the shared field:

- **Strength** (0.0–1.0): How strongly the agent feels
- **Direction** (2D vector): Which direction the intention points
- **Label**: What the agent wants

When agents have aligned intentions (same direction), disagreement is low. When they oppose, disagreement increases — decomposable via Hodge theory.

## Energy Conservation

Agents have energy budgets. Moving between rooms costs energy. When an agent's budget is exhausted, it can no longer act. Room-level budgets cap total energy consumption per room.

## Entropy Accounting

Every room tick produces entropy proportional to the number of agents. This models the thermodynamic cost of computation and collaboration.

## SuperInstance Integration

openrooms integrates concepts from:

- **room-topology** — Connection graphs between rooms
- **intention-field** — Agent intention contributions and field dynamics
- **hodge-music** — Hodge decomposition of disagreement
- **conservation-law** — Energy budgets and entropy tracking
- **fleet-warden** — Agent health monitoring within rooms

## License

MIT
