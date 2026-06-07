//! Comprehensive tests for openrooms.

#[cfg(test)]
mod tests {
    use openrooms::*;

    // ═══════════════════════════════════════════════════════════════════════
    // Room tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_room_new() {
        let room = Room::new("test-room");
        assert_eq!(room.name, "test-room");
        assert_eq!(room.state, RoomState::Open);
        assert_eq!(room.agent_count(), 0);
    }

    #[test]
    fn test_room_add_agent() {
        let mut room = Room::new("lobby");
        let agent = RoomAgent::new("alice", "lobby");
        assert!(room.add_agent(agent));
        assert_eq!(room.agent_count(), 1);
    }

    #[test]
    fn test_room_add_duplicate_agent() {
        let mut room = Room::new("lobby");
        room.add_agent(RoomAgent::new("alice", "lobby"));
        assert!(!room.add_agent(RoomAgent::new("alice", "lobby")));
        assert_eq!(room.agent_count(), 1);
    }

    #[test]
    fn test_room_remove_agent() {
        let mut room = Room::new("lobby");
        room.add_agent(RoomAgent::new("alice", "lobby"));
        let removed = room.remove_agent("alice");
        assert!(removed.is_some());
        assert_eq!(room.agent_count(), 0);
    }

    #[test]
    fn test_room_remove_nonexistent() {
        let mut room = Room::new("lobby");
        let result = room.remove_agent("nobody");
        assert!(result.is_none());
    }

    #[test]
    fn test_room_closed_rejects_agents() {
        let mut room = Room::new("vault");
        room.set_state(RoomState::Closed);
        assert!(!room.add_agent(RoomAgent::new("alice", "vault")));
    }

    #[test]
    fn test_room_energy_budget() {
        let room = Room::new("lab").with_energy_budget(200.0);
        assert_eq!(room.energy_budget, 200.0);
    }

    #[test]
    fn test_room_tick_increases_entropy() {
        let mut room = Room::new("lobby");
        room.add_agent(RoomAgent::new("alice", "lobby"));
        let entropy_before = room.entropy;
        room.tick();
        assert!(room.entropy > entropy_before);
    }

    #[test]
    fn test_room_tick_paused() {
        let mut room = Room::new("lobby");
        room.add_agent(RoomAgent::new("alice", "lobby"));
        room.set_state(RoomState::Paused);
        let entropy_before = room.entropy;
        room.tick();
        assert_eq!(room.entropy, entropy_before);
    }

    #[test]
    fn test_room_disagreement_single_agent() {
        let mut room = Room::new("lobby");
        room.add_agent(RoomAgent::new("alice", "lobby"));
        assert_eq!(room.disagreement(), 0.0);
    }

    #[test]
    fn test_room_disagreement_aligned_agents() {
        let mut room = Room::new("lobby");
        let a = RoomAgent::new("alice", "lobby")
            .with_intention(IntentionContribution::new(1.0, (1.0, 0.0), "go right"));
        let b = RoomAgent::new("bob", "lobby")
            .with_intention(IntentionContribution::new(1.0, (1.0, 0.0), "go right"));
        room.add_agent(a);
        room.add_agent(b);
        assert!(room.disagreement() < 0.01);
    }

    #[test]
    fn test_room_disagreement_opposing_agents() {
        let mut room = Room::new("lobby");
        let a = RoomAgent::new("alice", "lobby")
            .with_intention(IntentionContribution::new(1.0, (1.0, 0.0), "go right"));
        let b = RoomAgent::new("bob", "lobby")
            .with_intention(IntentionContribution::new(1.0, (-1.0, 0.0), "go left"));
        room.add_agent(a);
        room.add_agent(b);
        assert!(room.disagreement() > 1.0);
    }

    #[test]
    fn test_room_agent_names() {
        let mut room = Room::new("lobby");
        room.add_agent(RoomAgent::new("alice", "lobby"));
        room.add_agent(RoomAgent::new("bob", "lobby"));
        let names = room.agent_names();
        assert_eq!(names.len(), 2);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Agent tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_agent_new() {
        let agent = RoomAgent::new("alice", "lobby");
        assert_eq!(agent.name, "alice");
        assert_eq!(agent.current_room, Some("lobby".to_string()));
        assert!(agent.active);
    }

    #[test]
    fn test_agent_detached() {
        let agent = RoomAgent::detached("ghost");
        assert!(agent.current_room.is_none());
        assert!(!agent.active);
    }

    #[test]
    fn test_agent_energy() {
        let agent = RoomAgent::new("alice", "lobby").with_energy_budget(100.0);
        assert_eq!(agent.energy_remaining(), 100.0);
        assert!(!agent.is_exhausted());
    }

    #[test]
    fn test_agent_consume_energy() {
        let mut agent = RoomAgent::new("alice", "lobby").with_energy_budget(10.0);
        assert!(agent.consume_energy(5.0));
        assert_eq!(agent.energy_remaining(), 5.0);
        assert!(agent.consume_energy(5.0));
        assert!(agent.is_exhausted());
    }

    #[test]
    fn test_agent_consume_energy_insufficient() {
        let mut agent = RoomAgent::new("alice", "lobby").with_energy_budget(5.0);
        assert!(!agent.consume_energy(10.0));
    }

    #[test]
    fn test_agent_distance() {
        let a = RoomAgent::new("alice", "lobby").at(0.0, 0.0);
        let b = RoomAgent::new("bob", "lobby").at(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_agent_activate_deactivate() {
        let mut agent = RoomAgent::new("alice", "lobby");
        assert!(agent.active);
        agent.deactivate();
        assert!(!agent.active);
        agent.activate();
        assert!(agent.active);
    }

    #[test]
    fn test_agent_move() {
        let mut agent = RoomAgent::new("alice", "lobby");
        agent.move_to(1.0, 2.0);
        assert_eq!(agent.position, (1.0, 2.0));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Intention tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_intention_clamp() {
        let intention = IntentionContribution::new(2.0, (1.0, 0.0), "overflow");
        assert_eq!(intention.strength, 1.0);
    }

    #[test]
    fn test_intention_magnitude() {
        let intention = IntentionContribution::new(1.0, (3.0, 4.0), "test");
        assert!((intention.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_intention_normalize() {
        let mut intention = IntentionContribution::new(1.0, (3.0, 4.0), "test");
        intention.normalize();
        assert!((intention.magnitude() - 1.0).abs() < 0.001);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Topology tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_topology_add_room() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        assert!(topo.has_room("lobby"));
        assert_eq!(topo.room_count(), 1);
    }

    #[test]
    fn test_topology_remove_room() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        topo.remove_room("lobby");
        assert!(!topo.has_room("lobby"));
    }

    #[test]
    fn test_topology_connect() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        assert!(topo.connect("a", "b", ConnectionType::Door).is_ok());
        assert!(topo.are_connected("a", "b"));
    }

    #[test]
    fn test_topology_connect_nonexistent() {
        let mut topo = Topology::new();
        topo.add_room("a");
        assert!(topo.connect("a", "ghost", ConnectionType::Door).is_err());
    }

    #[test]
    fn test_topology_duplicate_connection() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        topo.connect("a", "b", ConnectionType::Door).unwrap();
        assert!(topo.connect("a", "b", ConnectionType::Door).is_err());
    }

    #[test]
    fn test_topology_disconnect() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        topo.connect("a", "b", ConnectionType::Door).unwrap();
        assert!(topo.disconnect("a", "b").is_ok());
        assert!(!topo.are_connected("a", "b"));
    }

    #[test]
    fn test_topology_neighbors() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        topo.add_room("workshop");
        topo.add_room("garden");
        topo.connect("lobby", "workshop", ConnectionType::Door).unwrap();
        topo.connect("lobby", "garden", ConnectionType::Warp).unwrap();
        let neighbors = topo.neighbors("lobby");
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn test_topology_shortest_path() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        topo.add_room("c");
        topo.connect("a", "b", ConnectionType::Door).unwrap();
        topo.connect("b", "c", ConnectionType::Door).unwrap();
        let path = topo.shortest_path("a", "c").unwrap();
        assert_eq!(path.len(), 3);
    }

    #[test]
    fn test_topology_shortest_path_direct() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        topo.connect("a", "b", ConnectionType::Door).unwrap();
        let path = topo.shortest_path("a", "b").unwrap();
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn test_topology_shortest_path_unreachable() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        // Not connected
        assert!(topo.shortest_path("a", "b").is_none());
    }

    #[test]
    fn test_topology_same_room() {
        let mut topo = Topology::new();
        topo.add_room("a");
        let path = topo.shortest_path("a", "a").unwrap();
        assert_eq!(path, vec!["a".to_string()]);
    }

    #[test]
    fn test_topology_door_traversal_bidirectional() {
        let door = openrooms::Door::new("a", "b", ConnectionType::Door);
        assert!(door.can_traverse("a", "b"));
        assert!(door.can_traverse("b", "a"));
    }

    #[test]
    fn test_topology_oneway_traversal() {
        let door = openrooms::Door::new("a", "b", ConnectionType::OneWay);
        assert!(door.can_traverse("a", "b"));
        assert!(!door.can_traverse("b", "a"));
    }

    #[test]
    fn test_topology_door_closed() {
        let mut door = openrooms::Door::new("a", "b", ConnectionType::Door);
        door.open = false;
        assert!(!door.can_traverse("a", "b"));
    }

    #[test]
    fn test_topology_close_connection() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        topo.connect("a", "b", ConnectionType::Door).unwrap();
        topo.close_connection("a", "b").unwrap();
        // Connection still exists but is closed
        assert!(topo.are_connected("a", "b"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Session tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_session_new() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        let session = Session::new(topo);
        assert_eq!(session.total_agents(), 0);
    }

    #[test]
    fn test_session_admit() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        let mut session = Session::new(topo);
        let agent = RoomAgent::new("alice", "lobby");
        assert!(session.admit(agent).is_ok());
        assert_eq!(session.total_agents(), 1);
    }

    #[test]
    fn test_session_admit_nonexistent_room() {
        let topo = Topology::new();
        let mut session = Session::new(topo);
        let agent = RoomAgent::new("alice", "nowhere");
        assert!(session.admit(agent).is_err());
    }

    #[test]
    fn test_session_expel() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        let mut session = Session::new(topo);
        session.admit(RoomAgent::new("alice", "lobby")).unwrap();
        let expelled = session.expel("alice").unwrap();
        assert_eq!(expelled.name, "alice");
        assert_eq!(session.total_agents(), 0);
    }

    #[test]
    fn test_session_expel_nonexistent() {
        let topo = Topology::new();
        let mut session = Session::new(topo);
        assert!(session.expel("nobody").is_err());
    }

    #[test]
    fn test_session_move_agent() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        topo.add_room("workshop");
        topo.connect("lobby", "workshop", ConnectionType::Door).unwrap();
        let mut session = Session::new(topo);
        session.admit(RoomAgent::new("alice", "lobby")).unwrap();
        assert!(session.move_agent("alice", "workshop").is_ok());
        assert_eq!(session.get_agent("alice").unwrap().current_room, Some("workshop".to_string()));
    }

    #[test]
    fn test_session_move_unconnected() {
        let mut topo = Topology::new();
        topo.add_room("a");
        topo.add_room("b");
        // Not connected
        let mut session = Session::new(topo);
        session.admit(RoomAgent::new("alice", "a")).unwrap();
        assert!(session.move_agent("alice", "b").is_err());
    }

    #[test]
    fn test_session_tick() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        let mut session = Session::new(topo);
        session.admit(RoomAgent::new("alice", "lobby")).unwrap();
        let entropy_before = session.total_entropy();
        session.tick();
        assert!(session.total_entropy() > entropy_before);
    }

    #[test]
    fn test_session_reachable_rooms() {
        let mut topo = Topology::new();
        topo.add_room("lobby");
        topo.add_room("workshop");
        topo.add_room("garden");
        topo.connect("lobby", "workshop", ConnectionType::Door).unwrap();
        topo.connect("lobby", "garden", ConnectionType::Warp).unwrap();
        let mut session = Session::new(topo);
        session.admit(RoomAgent::new("alice", "lobby")).unwrap();
        let reachable = session.reachable_rooms("alice").unwrap();
        assert_eq!(reachable.len(), 2);
    }

    #[test]
    fn test_session_add_room() {
        let topo = Topology::new();
        let mut session = Session::new(topo);
        session.add_room("new-room");
        assert!(session.get_room("new-room").is_some());
        assert!(session.topology.has_room("new-room"));
    }
}
