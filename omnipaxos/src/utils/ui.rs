use crate::{
    ballot_leader_election::{Ballot, Connectivity},
    storage::Entry,
    util::{LeaderState, NodeId},
};

/// The states of all the nodes in the cluster.
#[derive(Debug, Clone, Default)]
pub struct ClusterState {
    /// The accepted indexes of all the nodes in the cluster. The index of the vector is the node id.
    pub accepted_indexes: Vec<u64>,
    /// All the received heartbeats from the previous heartbeat round, including the current node.
    /// Represents nodes that are currently alive from the view of the current node.
    pub ballots: Vec<(Ballot, Connectivity)>,
}

impl<T> From<&LeaderState<T>> for ClusterState
where
    T: Entry,
{
    fn from(leader_state: &LeaderState<T>) -> Self {
        let mut accepted_indexes = leader_state.accepted_indexes.clone();
        // Add one empty entry at the beginning to make the index of the vector sync with node id.
        accepted_indexes.insert(0, 0);
        Self {
            accepted_indexes,
            ballots: vec![],
        }
    }
}

/// The states that are for UI to show.
pub struct OmniPaxosStates {
    /// The current ballot
    pub current_ballot: Ballot,
    /// The current leader
    pub current_leader: Option<NodeId>,
    /// The current decided index
    pub decided_idx: u64,
    /// All the received heartbeats from the previous heartbeat round, including the current node.
    /// Represents nodes that are currently alive from the view of the current node.
    pub ballots: Vec<(Ballot, Connectivity)>,
    /// The states of all the nodes in the cluster.
    pub cluster_state: ClusterState,
}