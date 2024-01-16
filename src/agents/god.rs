use arbiter_engine::machine::Behavior;
use durin_fault::FaultDisputeState;
use revm::primitives::Address;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// The [God] agent is the main agent of the simulation. It is responsible for tracking the state of the
/// DisputeGameFactory as well as created dispute games and communicating with the [durin_fault::FaultDisputeSolver]
/// to inform other agents of the correct interactions to make in live dispute games.
pub struct God {
    /// Mapping of [Address]es to [FaultDisputeState]s. This is used to track the state of dispute games that are
    /// currently in progress.
    fault_game_states: HashMap<Address, Arc<Mutex<FaultDisputeState>>>,
}

pub enum GodEvent {
    /// Triggered whenever a a new dispute is created.
    DisputeGameCreated(Address),
}

#[async_trait::async_trait]
impl Behavior<GodEvent> for God {
    async fn sync(&mut self) {}

    async fn process(&mut self, _: GodEvent) {}

    async fn startup(&mut self) {}
}
