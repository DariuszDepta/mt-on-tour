# Executing submessage `Wasm`

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
      "creator": "wasm1352mdds8fjq5mleedrxssxz6p20nwkmg6rq6g2",
      "data_hash": "5A03700335F7AACE95D0BCF84E2CA435B1ED57FB89BA956DE6BBF2F6B38107F1",
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

## Instantiate two contracts

```shell
$ wasmd tx wasm instantiate 1 '"none"' --label my-responder-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

```shell
$ wasmd tx wasm instantiate 1 '"none"' --label my-responder-2 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Check if both contracts were properly instantiated:
 
```shell
$ wasmd query wasm list-contract-by-code 1 -o json | jq
```

Output:

```json
{
  "contracts": [
    "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
    "wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Contract with address `wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d` will execute the `Add` message on the
contract with address `wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay`.

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"wasm_msg_execute_add":["wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay",263,87]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "5E122F1B86D5D34B5438CC628FC744769A6CC2976D0D09C118D12490EE076265",
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
$ wasmd query tx 5E122F1B86D5D34B5438CC628FC744769A6CC2976D0D09C118D12490EE076265 -o json | jq -r '.data' | xxd -p -r | decode_raw
```

Output:

```text
2 {
· 1: (44 bytes) "/cosmwasm.wasm.v1.MsgExecuteContractResponse"
· 2 {
· · 1: (105 bytes) '{"id":3,"msg_responses":[{"type_url":"/cosmwasm.wasm.v1.MsgExecuteContractResponse","value":"CgMzNTA="}]}'
· }
}
```

Check the returned value:

```shell
$ echo -n "CgMzNTA=" | base64 -d | decode_raw
```

Output:

```text
350
```

## Conclusions

> - The returned message type is `/cosmwasm.wasm.v1.MsgExecuteContractResponse`.
> - The returned submessage type is also `/cosmwasm.wasm.v1.MsgExecuteContractResponse`.
> - The returned submessage value is Base64 encoded protobuf containing JSON string **"350"**.

It looks like the value encoding is the following:

- Input: 350 (u64 integer).
- Conversion to JSON string: "350".
- Encoding with protobuf: 0a03333530 (hex)
- Encoding with Base64: CgMzNTA=
