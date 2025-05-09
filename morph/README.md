# Morph smart contract

#### Build the Wasm contract (not optimized)

```shell
$ cargo +1.81.0-x86_64-unknown-linux-gnu build --release --target wasm32-unknown-unknown --lib
```

#### Store the contract code on chain (not optimized)

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/morph.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

#### Build the Wasm contract with optimizer

```shell
$ docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.16.1
```

#### Store the contract code on chain (built with optimizer)

```shell
$ wasmd tx wasm store ./artifacts/morph.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "CD14641819AC2CFB55F5D537D7649C0425BC642D37BAAD57E45A8199DD875235",
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

Check the transaction if needed:

```shell
$ wasmd query tx CD14641819AC2CFB55F5D537D7649C0425BC642D37BAAD57E45A8199DD875235 -o json | jq
```

Output:

```json
{
  "height": "49",
  "txhash": "CD14641819AC2CFB55F5D537D7649C0425BC642D37BAAD57E45A8199DD875235",
  "codespace": "",
  "code": 0,
  "data": "124E0A262F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465526573706F6E7365122408011220099F2220392E3839E7DAB8FC77263135BC1D48FE28FCB841D6F95B47CF0E88D6",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "10000000",
  "gas_used": "1272621",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "body": {"rest of the message": "is omitted"}
  }
}
```

#### Check if the contract code was properly stored on chain

```shell
$ wasmd query wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1n67xm2va2s0ynvza3rt3pdef8rk2kf766pul37",
      "data_hash": "099F2220392E3839E7DAB8FC77263135BC1D48FE28FCB841D6F95B47CF0E88D6",
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


#### Instantiate the contract

The user that instantiates the contract is **bob**: 

```shell
$ wasmd tx wasm instantiate 1 {} --label morph-1 --no-admin --from bob --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "3E8BD519EC2C7137A2576EFA0D567812912DF0F65892E8C9A8C7243B45890C71",
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

Check the contract properties:

```shell
$ wasmd q wasm contract wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq
```

Output:

```json
{
  "address": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
  "contract_info": {
    "code_id": "1",
    "creator": "wasm18f80q4y78s3mfghargpeznx5lveu26ddxn3lkl",
    "admin": "",
    "label": "morph-1",
    "created": {
      "block_height": "89",
      "tx_index": "0"
    },
    "ibc_port_id": "",
    "ibc2_port_id": "",
    "extension": null
  }
}
```

#### Execute a message on the contract

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"appius"' --from bob --chain-id wte --keyring-backend=test -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "E970C4CEDFA35C0986553369B064B472FECA7BE27197F2B5B79069779B37E5A5",
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

Check the returned data:

```shell
$ wasmd query tx E970C4CEDFA35C0986553369B064B472FECA7BE27197F2B5B79069779B37E5A5 -o json | jq
```

Output:

```json
{
  "height": "132",
  "txhash": "E970C4CEDFA35C0986553369B064B472FECA7BE27197F2B5B79069779B37E5A5",
  "codespace": "",
  "code": 0,
  "data": "12590A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E736512290A277B226E616D65223A22417070697573222C22686569676874223A3139322C22616765223A33387D",
  "raw_log": "",
  "rest is omitted": true
}
```

Let's take a look into `data` field of this transaction:

```shell
$ echo -n "12590A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E736512290A277B226E616D65223A22417070697573222C22686569676874223A3139322C22616765223A33387D" | xxd -p -r | decode_raw
```

Output:

```text
2 {
· 1: (44 bytes) "/cosmwasm.wasm.v1.MsgExecuteContractResponse"
· 2 {
· · 1: (39 bytes) '{"name":"Appius","height":192,"age":38}'
· }
}
```

#### Execute another message on the contract

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"smileus"' --from bob --chain-id wte --keyring-backend=test -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "9328B3B53BEDAA725B3E016C3B7D58AC2D9D75A8D97F5785111B2F9D233F3652",
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

Check the returned data:

```shell
$ wasmd query tx 9328B3B53BEDAA725B3E016C3B7D58AC2D9D75A8D97F5785111B2F9D233F3652 -o json | jq
$ wasmd query tx E5F5981E4BFD7E8086D06F7E1F53DAF6366285B27E343D4AE80E5DF146F0CC3C -o json | jq
```

Output:

```json
{
  "height": "174",
  "txhash": "9328B3B53BEDAA725B3E016C3B7D58AC2D9D75A8D97F5785111B2F9D233F3652",
  "codespace": "",
  "code": 0,
  "data": "12420A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E736512120A107B22736D696C65223A22F09F9880227D",
  "raw_log": "",
  "rest is omitted": true
}
```

Let's take a look inside `data` field of this transaction:

```shell
$ echo -n "12420A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E736512120A107B22736D696C65223A22F09F9880227D" | xxd -p -r | decode_raw
```

Output:

```text
2 {
· 1: (44 bytes) "/cosmwasm.wasm.v1.MsgExecuteContractResponse"
· 2 {
· · 1: (16 bytes) '{"smile":"😀"}'
· }
}
```
