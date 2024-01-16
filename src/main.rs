#![allow(dead_code)]

use alloy_primitives::{b256, Address, U256};
use alloy_sol_types::{sol, SolCall, SolType};
use anyhow::{anyhow, Result};
use arbiter_core::{
    database::AnvilDump,
    environment::builder::{BlockSettings, EnvironmentBuilder},
};
use arbiter_engine::world::World;
use ethers::providers::Middleware;
use revm::db::{CacheDB, EmptyDB};

mod agents;
mod constants;
mod macros;

const SETUP_STATE: &str = include_str!("../state-dump/dump.json");

// testing; move later
mod dispute_game_factory {
    use alloy_sol_types::sol;

    sol! {
        function create(uint8 _gameType, bytes32 _claim, bytes calldata _extraData) external payable returns (address);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let anvil_dump: AnvilDump = serde_json::from_str(SETUP_STATE)?;
    let db: CacheDB<EmptyDB> = anvil_dump.try_into()?;
    let mut world = World::new_with_env("dispute_game", EnvironmentBuilder::new().db(db).build());
    world.environment.parameters.block_settings = BlockSettings::UserControlled;

    // Start off by forwarding to a non-zero block / timestamp.
    let agent = world.create_agent("orchestrator");
    agent.client.update_block(
        U256::from(1 << 5).to_be_bytes(),
        U256::from(1 << 5).to_be_bytes(),
    )?;

    // Create a mock dispute game that claims an output root for L2 block #0xFF
    let create_tx = dispute_game_factory::createCall {
        _gameType: 0,
        _claim: b256!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
        _extraData: <sol!(uint256)>::abi_encode(&U256::from(0xFF)),
    };
    let receipt = send_tx!(
        agent,
        constants::DGF_PROXY_ADDR,
        create_tx.abi_encode().into()
    )
    .await?;
    dbg!(&receipt);

    let dispute_game_proxy = receipt
        .logs
        .first()
        .ok_or(anyhow!("no logs"))?
        .topics
        .get(1)
        .ok_or(anyhow!("no topics"))?;
    dbg!(Address::from_word(
        dispute_game_proxy.as_fixed_bytes().into()
    ));

    Ok(())
}
