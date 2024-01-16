#![allow(dead_code)]

use anyhow::Result;
use arbiter_core::{database::AnvilDump, environment::{builder::EnvironmentBuilder, cheatcodes::{Cheatcodes, CheatcodesReturn}}};
use arbiter_engine::world::World;
use revm::db::{CacheDB, EmptyDB};

mod agents;
mod constants;
mod deployer;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil_dump: AnvilDump = serde_json::from_str(include_str!("../state-dump/dump.json"))?;
    let db: CacheDB<EmptyDB> = anvil_dump.try_into()?;
    let env = EnvironmentBuilder::new().db(db).build();
    let mut world = World::new_with_env("dispute_game", env);

    let agent = world.create_agent("foo");
    let dgf = agent.client.apply_cheatcode(Cheatcodes::Access { address: ethers::types::H160::from_slice(constants::DGF_PROXY_ADDR.as_slice()) }).await?;

    match dgf {
        CheatcodesReturn::Access { info, account_state, storage } => {
            dbg!(info, account_state, storage);
        }
        _ => panic!("wrong")
    }

    Ok(())
}
