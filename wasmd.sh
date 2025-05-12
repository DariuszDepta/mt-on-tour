#!/usr/bin/env bash

CHAIN_ID=${CHAIN_ID:-wte}

cd ~ || exit
rm -rf ~/.wasmd
rm -rf ~/wasmd
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd || exit
make install
wasmd init wte --chain-id="$CHAIN_ID"
wasmd keys add alice --keyring-backend=test
wasmd keys add bob --keyring-backend=test
wasmd keys add cecil --keyring-backend=test
wasmd keys add dave --keyring-backend=test
wasmd genesis add-genesis-account validator "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account dave "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account edwin "1000000000000stake" --keyring-backend=test
wasmd genesis gentx validator "250000000stake" --chain-id=$CHAIN_ID --amount="250000000stake" --keyring-backend=test
wasmd genesis collect-gentxs
wasmd start
