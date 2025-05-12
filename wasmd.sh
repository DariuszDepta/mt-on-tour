#!/usr/bin/env bash

MONIKER=${MONIKER:-wte}
CHAIN_ID=${CHAIN_ID:-wte}
KEYRING=${KEYRING:-test}

cd ~ || exit
rm -rf ~/.wasmd
rm -rf ~/wasmd
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd || exit
make install
wasmd init "$MONIKER" --chain-id="$CHAIN_ID"
wasmd keys add validator --keyring-backend="$KEYRING"
wasmd keys add alice --keyring-backend="$KEYRING"
wasmd keys add bob --keyring-backend="$KEYRING"
wasmd keys add dave --keyring-backend="$KEYRING"
wasmd keys add edwin --keyring-backend="$KEYRING"
wasmd genesis add-genesis-account validator "1000000000000stake" --keyring-backend="$KEYRING"
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend="$KEYRING"
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend="$KEYRING"
wasmd genesis add-genesis-account dave "1000000000000stake" --keyring-backend="$KEYRING"
wasmd genesis add-genesis-account edwin "1000000000000stake" --keyring-backend="$KEYRING"
wasmd genesis gentx validator "250000000stake" --chain-id="$CHAIN_ID" --amount="250000000stake" --keyring-backend="$KEYRING"
wasmd genesis collect-gentxs
wasmd start
