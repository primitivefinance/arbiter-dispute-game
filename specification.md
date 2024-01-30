# Dispute Game Simulation Specifications

This document aims to dial in on a specification and scope for the Primitive team to audit the disputed game with Arbiter simulations. For the purpose of this document we consider a `Claim` to be a move in the dispute game.

## Scope
We propose to define two core deliverables for the OP labs team. Both deliverables will evolve from this repository.
- A report of our findings and analysis of the dispute game located in the `security_report` directory.

- A simulation of the dispute game located in the `src` directory. We want to run 100M Sims on a (single or many but takes about 45min to compute) connon trace (on average 1.5b instructions) with at least 5 agent behaviors. The [Simulation Specification](#simulation-specification) section defines the simulation details below.

## The Dispute Game
The fault-proof dispute game is an interactive proof system used to prove the validity of the layer two states on layer one. Any number of players can play the game. The game has a 7-day chess clock and terminates when the clock runs out. Each player moves by making a `Claim` (see dispute game interface) about an index in a cannon execution trace. If no opposing move is made, the claim implicitly resolves to true. Each player must post a bond to make a claim. If a player makes a false claim and the game resolves, their bond is forfeited and paid to the player who made the opposing correct claim.

- [Dispute Game Interface](https://github.com/ethereum-optimism/specs/blob/main/specs/dispute-game-interface.md)
- [Fault Dispute Game](https://github.com/ethereum-optimism/specs/blob/main/specs/fault-dispute-game.md)

## Arbiter Simulation

Arbiter is an interface over an EVM sandbox built around the rust evm. Arbiter enables agent-based simulations to model the behavior of different externally owned accounts that would interact with a smart contract or system of smart contracts. This allows us to perturb other variables in the system and observe the effects on the system to discover potential attack vectors and anomalous behavior. 

### Simulation Specification
These are the details of the simulation design.

#### Initial State
We configure a simulation by defining an initial state of the system we want before iterating agent behavior over the model. Traditionally, we have used deployer-style agents to deploy the smart contracts we want to simulate with the correct initial state. This has historically involved generating rust contract bindings and interfacing with contracts through these bindings.

Many solidity developers deploy their infrastructure with forge scripts. We have been able to grab the state that is initialized by a forge script on an anvil instance and dump it into a JSON file. This JSON file can then be loaded into an Arbiter instance to initialize the state of the system. The benefit of this is that we can initialize the system's state the same way it would be initialized in production. This allows us to uncover any particular bugs that may be present in the forge script. While this is nice with the optimism mono repo (like a million contracts), it does not mean we will use the sol macro for communicating with the contracts (examples in the `src` directory. 

![Forge Script]assets/image.png)

This work has already been done, and the bash and forge scripts can be seen [here](state-dump/dump.sh) and [here](state-dump/monorepo/packages/contracts-bedrock/scripts/fpac/FPACOPS.sol)

#### Involved Contracts
- `PreImageOracle`: It is my understanding that this is interacted with the solver and not our agents (need to check with Ben)
- `DisputeGameFactory`: This contract is used to create dispute game instances which enable the dispute game to be played. The game is played when players submit claims. A Claim is only submitted in opposition to the current state. 
- `MIPs`: MIPs is the mips32 instruction set architecture in solidity. This is what produces a cannon trace which is the state of L2 that players make claims about.

#### Agents

Agents are defined by their behaviors. We define a behavior trait as follows
```rust
/// The [`Behavior`] trait is the lowest-level functionality that will be used
/// by a [`StateMachine`]. This constitutes what each state transition will do.
#[async_trait::async_trait]
pub trait Behavior<E>: Send + Sync + 'static {
    /// Used to bring the agent back up to date with the latest state of the
    /// world. This could be used if the world was stopped and later restarted.
    async fn sync(&mut self, _messager: Messager, _client: Arc<RevmMiddleware>) {}

    /// Used to start the agent.
    /// This is where the agent can engage in its specific start-up activities
    /// that it can do, given the current state of the world.
    async fn startup(&mut self) {}

    /// Used to process events.
    /// This is where the agent can engage in its specific processing
    /// of events that can lead to actions being taken.
    async fn process(&mut self, event: E) -> Option<MachineHalt>;
}
```

The dispute game simulation will have the following agents.

- **Game Master:** Responsible for reading the state of a dispute game instance (created by the dispute game factory contract) and sending the new claims to Durin, the dispute game solver. Durin will return a correct move. The Game master will then send the correct move to all other agents (game players) in the simulation. The game master is the only agent who is not a player in the game. The Game master is the most complicated agent in the simulation. It will be responsible for the following:
    - Reading the state of the dispute game instance
    - Sending the state to Durin
    - Receiving the correct move from Durin
    - Sending the correct move to all other agents

- **Honest Agent:** Responsible for making correct claims on the dispute game instance. This is the most straightforward agent in the simulation. It will be responsible for the following:
    - Receiving the correct move from the game master
    - Making a correct claim on a dispute game instance

- **Malicious Agent:** Responsible for making incorrect claims on the dispute game instance. This agent is also quite simple but can have many different variations. It will be responsible for the following:
    - Making an incorrect claim on a dispute game instance
    - Creating a game by submitting a dishonest claim

Here is a diagram of the agent interactions in the simulation.

![Alt text](assets/image-1.png)

#### Durin

[Durin](https://github.com/anton-rs/durin) is a game solver the master will querry. Durin will solve a [cannon trace](https://github.com/ethereum-optimism/optimism/tree/develop/cannon) for the given claim and return the correct move to the game master. Druin solves the game by bisecting the cannon trace.

#### Main simulation loop
We have everything we need to run the main simulation loop with these agents and Durin. Once we get this iteration off the ground, we can start getting creative with malicious agent behavior and see the stress tests under which we can put the dispute game.
![Alt text](assets/image-2.png)

## Open questions
- What is the status of durin. Does it block us on anything? Should it be included in the scope?
- What are the externally owned accounts that interact with the pre-image oracle? 