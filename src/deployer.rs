use std::sync::RwLockWriteGuard;

use anyhow::{Result, anyhow};
use arbiter_core::database::ArbiterDB;
use arbiter_engine::{machine::Behavior, messager::Message};
use revm::db::{EmptyDB, CacheDB};

/// Deploys the dispute game system to the passed database.
pub(crate) fn deploy_contracts(db: RwLockWriteGuard<'_, CacheDB<EmptyDB>>) -> Result<()> {
    Ok(())
}
