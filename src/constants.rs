//! Constants used throughout the simulation.

use revm::primitives::{Address, address};

/// The [arbiter_engine::agent::Agent] ID for the deployer actor. 
pub const DEPLOYER_ID: &str = "deployer";

/// The [Address] of the dispute game factory proxy.
pub const DGF_PROXY_ADDR: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

/// The [Address] of the preimage oracle contract.
pub const PREIMAGE_ORACLE_ADDR: Address = address!("f512B6a94bdc343db982bd5De4cCe80A9Ff59Fa6");

/// The [Address] of the MIPS VM contract.
pub const MIPS_ADDR: Address = address!("4c8C6a9FeD6Bcfc7c6c1872A070c40A8107b1281");
