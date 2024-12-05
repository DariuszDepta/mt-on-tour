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
> The address of the user that stored the source code is: **wasm1tr2etcsklkc802qflx9qeh78pfs8tvar52e4s4**.<br/>
> The identifier of the source code is **1**.  
