# Introduction example

Objectives:

- Show how to use cw-multi-test as a dependency.
- Show how to organize code for testing.
- Show how to write tests.

## Recreate the chain

Run [wasmd.sh](../wasmd.sh) script.

## Interact with the chain

### Check the list of source codes stored on the chain:

```shell
$ wasmd q wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

### Store the contract's source code on chain

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/intro.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Output: 

```json
{
  "height": "0",
  "txhash": "E1E9CAD1E26208A6151209B925E6A5F3E01007549FB8C4C7BACDD3A059FF5889",
  "codespace": "",
  "code": 0,
  "data": "",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "0",
  "gas_used": "0",
  "tx": null,
  "timestamp": "",
  "events": []
}
```

Check if the code was properly stored:

```shell
$ wasmd q wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1tr2etcsklkc802qflx9qeh78pfs8tvar52e4s4",
      "data_hash": "E3523348FF69519F913FBB12DC5660161ECB2978E603E909925DD87ABCC6A229",
      "instantiate_permission": {
        "permission": "Everybody",
        "addresses": []
      }
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

> NOTES:<br/>
> The address of the user that stored the source code is `alice`: **wasm1tr2etcsklkc802qflx9qeh78pfs8tvar52e4s4**.<br/>
> The identifier of the source code is **1**.  

### Instantiate the contract

The user that instantiates the contract is **bob**: 

```shell
$ wasmd tx wasm instantiate 1 {} --label intro-1 --no-admin --from bob --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "C16D20E8484EC6D25DBCB54A056215DB44EF106CF4D1EA7048ECFD817C0BECD9",
  "codespace": "",
  "code": 0,
  "data": "",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "0",
  "gas_used": "0",
  "tx": null,
  "timestamp": "",
  "events": []
}
```

Check if the contract was instantiated:

```shell
$ wasmd query wasm list-contract-by-code 1 -o json | jq
```

```json
{
  "contracts": [
    "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Get the metadata of the contract:

```shell
$ wasmd q wasm contract wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq
```

```json
{
  "address": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
  "contract_info": {
    "code_id": "1",
    "creator": "wasm190jdj78ct26zjavln7l6j4t2xm3vyfrpc52z6p",
    "admin": "",
    "label": "intro-1",
    "created": {
      "block_height": "178",
      "tx_index": "0"
    },
    "ibc_port_id": "",
    "extension": null
  }
}
```

Get the metadata of the contract's code:

```shell
$ wasmd query wasm code-info 1 -o json | jq 
```

```json
{
  "code_id": "1",
  "creator": "wasm1tr2etcsklkc802qflx9qeh78pfs8tvar52e4s4",
  "checksum": "E3523348FF69519F913FBB12DC5660161ECB2978E603E909925DD87ABCC6A229",
  "instantiate_permission": {
    "permission": "Everybody",
    "addresses": []
  }
}
```

> NOTES:<br/>
> The creator of the contract instance is `bob`: **wasm190jdj78ct26zjavln7l6j4t2xm3vyfrpc52z6p**.<br/>
> The creator of the source code the contract was instantiated is `alice`: **wasm1tr2etcsklkc802qflx9qeh78pfs8tvar52e4s4**.<br>
> The checksum can be verified this way:
> ```shell
> $ sha256sum ./target/wasm32-unknown-unknown/release/intro.wasm | awk '{print toupper($1)}'
> ```
> The result should be: `E3523348FF69519F913FBB12DC5660161ECB2978E603E909925DD87ABCC6A229`. 
