[MultiTest](https://crates.io/crates/cw-multi-test) | Examples

# Nayel smart contract

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

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/nayel.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "5D60807D949B49A881AB754B8D3D54342B0CCBB488004168CF9D41E65A0DE52F",
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

```shell
$ wasmd q wasm list-code -o json | jq 
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1uw4zkfu3l3eh7p3xv4c789qllu9pqqms4na8yu",
      "data_hash": "6972C3D14244230703B16B0DAA5EFF0FB8A0724D03FD2E1EB8051EC8DD905B0B",
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

```shell
$ wasmd tx wasm instantiate 1 {} --label nayel-1 --no-admin --from bob --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "FD4EB7C01DA2FA291912B34A5FBBA9B094672AF7610476C14883AB7E9474C45E",
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

```shell
$ wasmd query tx FD4EB7C01DA2FA291912B34A5FBBA9B094672AF7610476C14883AB7E9474C45E -o json | jq '.data' | xxd -r -p | protoc --decode_raw
```

Output:

```text
2 {
  1: "/cosmwasm.wasm.v1.MsgInstantiateContractResponse"
  2 {
    1: "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
    2: "{\"code_creator_address\":\"wasm1uw4zkfu3l3eh7p3xv4c789qllu9pqqms4na8yu\",\"code_id\":null,\"contract_creator_address\":null,\"contract_address\":\"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d\",\"err_contract_info\":\"Generic error: Querier system error: No such contract: wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d\",\"err_code_info\":null,\"err_self_query\":\"Generic error: Querier system error: No such contract: wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d\"}"
  }
}
```

```shell
$ wasmd query tx FD4EB7C01DA2FA291912B34A5FBBA9B094672AF7610476C14883AB7E9474C45E -o json | jq '.data' | xxd -r -p | decode_raw
```

Output:

```text
2 {
· 1: (48 bytes) "/cosmwasm.wasm.v1.MsgInstantiateContractResponse"
· 2 {
· · 1: (63 bytes) "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
· · 2: (502 bytes) '{"code_creator_address":"wasm1uw4zkfu3l3eh7p3xv4c789qllu9pqqms4na8yu","code_id":null,"contract_creator_address":null,"contract_address":"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d","err_contract_info":"Generic error: Querier system error: No such contract: wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d","err_code_info":null,"err_self_query":"Generic error: Querier system error: No such contract: wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"}'
· }
}
```

```json
{
  "code_creator_address": "wasm1uw4zkfu3l3eh7p3xv4c789qllu9pqqms4na8yu",
  "code_id": null,
  "contract_creator_address": null,
  "contract_address": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
  "err_contract_info": "Generic error: Querier system error: No such contract: wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
  "err_code_info": null,
  "err_self_query": "Generic error: Querier system error: No such contract: wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
}
```

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

## Final notes

Calling

```rust
let contract_addr = env.contract.address;
deps.querier.query_wasm_contract_info(contract_addr);
```

inside `instantiate` entry-point returns an error and this should be fixed in `wasmd`.

Calling

```rust
let code_id = 1u64;
deps.querier.query_wasm_code_info(code_id);
```

inside `instantiate` entry-point works in `wasmd`, but one has to know the `code_id`.
The `code_id` for a contract may only be retrieved using `query_wasm_contract_info`
but this does not work in `wasmd`.

Calling `query_wasm_smart` inside `instantiate` function for the instantiated contract  
should return an error as it does now, because the contract is not yet fully instantiated.

