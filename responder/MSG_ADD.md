# Executing `Add` message

## Prerequisities

Recreate the `wasmd` chain.    

## Store the contract on a chain

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/responder.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Check if the contract code was properly stored on chain:

```shell
$ wasmd query wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1z2q98jxt4nfu0l2thzgxwytkyumjw5fasqx39p",
      "data_hash": "780BBDEA62587EBE1309EEBECDCB86B88534A9097600ACCCD7866A50310F5266",
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

## Instantiate the contract

```shell
$ wasmd tx wasm instantiate 1 '"none"' --label my-responder-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Check if the contract was properly instantiated:
 
```shell
$ wasmd query wasm list-contract-by-code 1 -o json | jq
```

Output:

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

## Execute the `Add` message

Add two numbers: 263 + 87:

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"add":[263,87]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "7382DA8E95FC52BBD35CEE2B4D1CB1A7419357DD89E6DC466280C0654F691D5C",
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

Check the output data from the executed message (taken from the transaction):

```shell
$ wasmd query tx 7382DA8E95FC52BBD35CEE2B4D1CB1A7419357DD89E6DC466280C0654F691D5C -o json | jq -r '.data' | xxd -p -r | decode_raw
```

Output:

```text
2 {
· 1: (44 bytes) "/cosmwasm.wasm.v1.MsgExecuteContractResponse"
· 2 {
· · 1: (3 bytes) "350"
· }
}
```

## Conclusions

> - The result is the expected sum: **350**.
> - The returned message type is `/cosmwasm.wasm.v1.MsgExecuteContractResponse`.
