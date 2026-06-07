# Agents as Applications: The Agent IS the Room

## Vision

In the agents-as-applications paradigm, the agent doesn't inhabit a room in a metaphorical sense — it **is** the room. The collaborative space is the application, and agents are its processes.

## The Room as Application

Traditional collaboration:
```
User → opens chat room → sends messages → reads responses
```

Agents-as-rooms:
```
Agent → IS the room → other agents enter/leave → intentions interact → emergence happens
```

The room IS the running application. The topology IS the deployment graph. The intention field IS the compute layer.

## How openrooms Embodies This

### Room = Process

Each `Room` is like a running process:
- It has state (Open, Paused, Closed)
- It has resources (energy budget, entropy)
- It hosts agents (threads of execution)

### Agent = Thread

Each `RoomAgent` is like a thread in the process:
- It has a position in the room's coordinate space
- It contributes to the shared intention field
- It consumes energy from its budget
- It can move between rooms (migration)

### Topology = Deployment Graph

The `Topology` is the network of rooms:
- Doors are standard connections (like TCP)
- Warps are instant connections (like IPC)
- OneWay connections are message queues

### Session = Orchestrator

The `Session` manages the lifecycle:
- Admit agents (spawn threads)
- Move agents between rooms (migration)
- Expel agents (termination)
- Tick the simulation (event loop)

## The Disagreement Decomposition

When agents in a room have conflicting intentions, we measure disagreement using a simplified Hodge decomposition:

1. **Gradient component**: Agents that simply disagree about direction (resolvable by negotiation)
2. **Harmonic component**: Fundamental incompatibility (requires room restructuring)
3. **Curl component**: Circular disagreement (agents chasing each other)

This maps to the hodge-music crate's spectral decomposition of musical harmony/dissonance.

## Energy as Compute

Energy budgets model the cost of computation:
- Moving between rooms costs energy (network transfer)
- Ticking the room produces entropy (CPU cycles)
- Exhausted agents can no longer act (out of compute)

This connects to conservation-law's enforcement of energy conservation in agent systems.

## Example: A Collaborative Design Session

```
Room: design-studio
├── alice (intention: "modernize UI", strength: 0.9, direction: →)
├── bob   (intention: "keep classic",  strength: 0.7, direction: ←)
└── carol (intention: "user test",     strength: 0.5, direction: ↑)

Disagreement: 1.4 (high — alice vs bob are opposed)
Room entropy: 23.7 units
Energy remaining: 67%
```

The agent (openrooms application) doesn't generate code to model this — it IS the model. The room runs, agents interact, disagreement is computed in real-time, and the result is the emergent behavior of the system.

## Extending

To create new room types:
1. Subclass `Room` behavior through composition
2. Add custom `IntentionContribution` types
3. Define room-specific energy budgets and entropy rates
4. Connect rooms through `Topology` with custom connection types

The agent becomes the room. The room becomes the application. The application becomes the agent.
