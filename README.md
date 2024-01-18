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
