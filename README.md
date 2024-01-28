# `dispute-game-arbiter`

Arbitin' the dispute game.

## How to Run

1. Before anything, generate the initial state dump with the `dump.sh` script in `state-dump`. This will generate a dump
   file of the `anvil` devnet state after deploying the Fault Proof contracts, giving us the initial setup state of the
   `revm` database in Arbiter.

```sh
git submodule update --init --recursive && \
    ./state-dump/dump.sh
```

2. Run the agent simulation with `cargo r`
# arbiter-dispute-game

# latex report
to generate the pdf security report from the main.tex file, run the `./generate_report.sh` script. This will generate a `main.pdf` file in `security_report` directory.


## Dispute Game Curriculum

- [Optimistic Rollup](https://ethereum.org/ig/developers/docs/scaling/optimistic-rollups): High level introduction to Optimistic Rollups
- [dispute-games](https://blog.oplabs.co/dispute-games/): Introduction to the optimism dispute game
- [Official Dispute Game Specification](https://github.com/ethereum-optimism/optimism/blob/develop/specs/fault-dispute-game.md)
- [Dispute Game Video](https://www.youtube.com/watch?v=nIN5sNc6nQM)
- [Bondorama](https://www.notion.so/oplabs/Bondorama-886cd1cfefcc44649f3e16f47d9a4477?pvs=4): Current Bond pricing for the game 

### Stage 2 Decentralization
- [Stage 2 boogaloo](https://hackmd.io/@clabby/BkZbT5hD6): What is stage two decentralization? 
- [Durin](https://github.com/anton-rs/durin): Durin is a Rust Solver for the Dispute Game
