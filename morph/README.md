# Morph smart contract

#### Build the contract

```shell
$ cargo build --release --lib --target wasm32-unknown-unknown

$ docker run --rm -v "$(pwd)":/code --mount type=volume,source="devcontract_cache_ibc2",target=/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.15.0 .


$ docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.16.1
```

#### Store the contract code on chain

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/morph.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
$ wasmd tx wasm store ./artifacts/morph.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "FF4F10FC0755B255FEF66F874DAFFF7E498E180EC390DA9286D7B33FB6B00133",
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

#### Check if the contract code was properly stored on chain

```shell
$ wasmd query tx FF4F10FC0755B255FEF66F874DAFFF7E498E180EC390DA9286D7B33FB6B00133 -o json | jq
$ wasmd query tx 281EDB6E476EC57DC0D1EFBE5707CF738684A0F5469B10FD8E2F86471C5200BF -o json | jq
```

```shell
$ wasmd query wasm list-code -o json | jq
```

Output



The user that instantiates the contract is **bob**: 

```shell
$ wasmd tx wasm instantiate 1 {} --label morph-1 --no-admin --from bob --chain-id wte --keyring-backend=test -o json -y | jq
```

```shell
$ wasmd q wasm contract wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq
```

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"appius"' --from bob --chain-id wte --keyring-backend=test -y -o json | jq


$ wasmd query tx BB13E4F0ABBA61435F56F66FA10DE494BABAD959424BF8CBED3D6BF14259F762 -o json | jq
```


```text
"12590A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E736512290A277B226E616D65223A22417070697573222C22686569676874223A3139322C22616765223A33387D"
```

And let's take a look into `data` field of this transaction:

```shell
$ echo -n "12590A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E736512290A277B226E616D65223A22417070697573222C22686569676874223A3139322C22616765223A33387D" | xxd -p -r | decode_raw
```

```text
2 {
· 1: (44 bytes) "/cosmwasm.wasm.v1.MsgExecuteContractResponse"
· 2 {
· · 1: (39 bytes) '{"name":"Appius","height":192,"age":38}'
· }
}
```

```shell
$ echo -n "7b226e616d65223a22417070697573222c22686569676874223a3139322c22616765223a33387d" | xxd -p -r | decode_raw
$ echo -n "0a277b226e616d65223a22417070697573222c22686569676874223a3139322c22616765223a33387d" | xxd -p -r | decode_raw
```
