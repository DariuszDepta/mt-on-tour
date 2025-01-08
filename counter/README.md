# Counter smart contract

## Storing contract on chain

Store the contract code on chain:

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/counter.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "B48799D8F5EFFD262F4716C6C56783C10371D85ED49AA5E6C45DAD591103EB75",
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

> NOTE: It looks like the wasm binary is compressed and base64 encoded when in transaction displayed as JSON. 

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
      "creator": "wasm1a8y6j9txcfyz6zy5ff0m0xsu3fqyxyy5u2nq2n",
      "data_hash": "D663636B86C09DD6B2D431846E600D62A1E3EDA4641918C8C8E261FB2600D74C",
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

## Instantiating the contract

Instantiate a contract with initial value set to 0: 

```shell
$ wasmd tx wasm instantiate 1 '"zero"' --label my-counter-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "95330EACCBA9BE9EDD0215BA0558006908853F2A879C949DA7D26DA0DBD4EB76",
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

## Instantiating the contract with predictable address

After the instantiating message, there is a salt provided as a hex string.

```shell
$ wasmd tx wasm instantiate2 1 '"zero"' 010203 --label my-counter-2 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "51A34B3DD984594E8F046FDC26EC4C3DA4016823130247E1FADFF3B12659F6A4",
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
    "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
    "wasm1l6ug8zcdvejlas0st7qyvz5lv5vm5ppqdzx0mjsk0cgyc6ktsa6sp32c0y"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

A new contract instance was created, with the address `wasm1l6ug8zcdvejlas0st7qyvz5lv5vm5ppqdzx0mjsk0cgyc6ktsa6sp32c0y`.

Creating another instance with different salt should work:

```shell
$ wasmd tx wasm instantiate2 1 '"zero"' 01020304 --label my-counter-2 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "ED77A88F2AF56F0E3CD94A4ADC07B5958C14799B4B9B472470D3BCF27FB15F61",
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
    "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
    "wasm1l6ug8zcdvejlas0st7qyvz5lv5vm5ppqdzx0mjsk0cgyc6ktsa6sp32c0y",
    "wasm1mwzkakc2j5d57fwpgy20ptwj9h5h6kgytn900wkwtxdcgg2n2hpq4r2x55"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

A new instance was created, with the address `wasm1mwzkakc2j5d57fwpgy20ptwj9h5h6kgytn900wkwtxdcgg2n2hpq4r2x55`.

Instantiating a contract with the same salt should fail:

```shell
$ wasmd tx wasm instantiate2 1 '"zero"' 010203 --label my-counter-2 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "806457A6B112070129019C2276949B2BF2D7AA9A20E14C9781B194A8CBA8972C",
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

Let's check the reported error:

```shell
$ wasmd query tx "806457A6B112070129019C2276949B2BF2D7AA9A20E14C9781B194A8CBA8972C" -o json | jq .raw_log
```

Output:

```json
"failed to execute message; message index: 0: contract address already exists, try a different combination of creator, checksum and salt: duplicate"
```

## Querying the current counter value

Query the current value of the counter contract (should be 0):

```shell
$ wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' -o json | jq
```

Output:

```json
{
  "data": {
    "value": 0
  }
}
```

## Setting a new counter value

Set the value of the counter to 53:

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"set":53}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "189DA916D38E1E338EEFA9F113A31335BCB510923C0407399AB24DFF56D38B0C",
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

Query the current value of the counter contract (should be 53):

```shell
$ wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' -o json | jq
```

Output:

```json
{
  "data": {
    "value": 53
  }
}
```

## Querying the counter value using gRPC

Using `wasmd` there was a string `'"value"'` passed as query data. When using gRPC this string must be encoded as Base64 string, like this:

```shell
$ echo -n '"value"' | base64
```

Output:

```text
InZhbHVlIg==
```

Because the protobuf definition of the state query looks like this:

```protobuf
syntax = "proto3";

message QuerySmartContractStateRequest {
  string address = 1;
  bytes query_data = 2;
}
``` 

So the full `grpcurl` command looks like this:

```shell
$ grpcurl -plaintext -d '{"address":"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d","query_data":"InZhbHVlIg=="}' localhost:9090 cosmwasm.wasm.v1.Query/SmartContractState
```

Output:

```json
{
  "data": "eyJ2YWx1ZSI6NTN9"
}
```

The output data is encoded as Base64 string, so it can be decoded this way: 

```shell
grpcurl -plaintext -d '{"address":"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d","query_data":"InZhbHVlIg=="}' localhost:9090 cosmwasm.wasm.v1.Query/SmartContractState | jq .data -r | base64 -d
```

Output:

```json
{"value":53}
```

## Querying the counter value using RPC

The protobuf definition of the state query looks like this (the same as for gRPC):

```protobuf
syntax = "proto3";

message QuerySmartContractStateRequest {
  string address = 1;
  bytes query_data = 2;
}
```

In this case the contract address is also `wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d`,
and the query data is a plain JSON string `"value"`.

After serializing the protobuf to HEX format, the query data is:

```text
0A3F7761736D3134686A32746176713866706573647778786375343472747933686839307668756A7276636D73746C347A723374786D667677397330706867346412072276616C756522
```

It is simple to verify by calling:

```shell
$ echo -n "0A3F7761736D3134686A32746176713866706573647778786375343472747933686839307668756A7276636D73746C347A723374786D667677397330706867346412072276616C756522" | xxd -r -p | decode_raw
``` 

Output:

```text
1: (63 bytes) "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
2: (7 bytes) '"value"'
```

The data for RPC query looks like this:

```json
{
  "jsonrpc": "2.0",
  "id": "102a24e1-5a0b-48c1-8a3d-7a057f7a13ec",
  "method": "abci_query",
  "params": {
    "path": "/cosmwasm.wasm.v1.Query/SmartContractState",
    "data": "0A3F7761736D3134686A32746176713866706573647778786375343472747933686839307668756A7276636D73746C347A723374786D667677397330706867346412072276616C756522",
    "height": "0",
    "prove": false
  }
}
```

When this JSON is saved to file named `data.json`, then the `curl` command looks like this:

```shell
$ curl -s 'http://localhost:26657' --data @data.json | jq
```

Output:

```json
{
  "jsonrpc": "2.0",
  "id": "102a24e1-5a0b-48c1-8a3d-7a057f7a13ec",
  "result": {
    "response": {
      "code": 0,
      "log": "",
      "info": "",
      "index": "0",
      "key": null,
      "value": "Cgx7InZhbHVlIjo1M30=",
      "proofOps": null,
      "height": "4273",
      "codespace": ""
    }
  }
}
```

The returned value is a protobuf encoded in Base64:

```shell
$ echo -n "Cgx7InZhbHVlIjo1M30=" | base64 -d | decode_raw
```

Output:

```text
1: (12 bytes) '{"value":53}'
```

So as expected, the returned counter value is `53`.

## Protobuf (de)serialized manually

### Serialization

The protobuf definition:

```protobuf
syntax = "proto3";

message QuerySmartContractStateRequest {
  string address = 1;
  bytes query_data = 2;
}
```

- `address` value (string): `wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d`.
- `query_data` value (bytes) `"value"`.

Taken from [protobuf specification](https://protobuf.dev/programming-guides/encoding):

> The “tag” of a record is encoded as a varint formed from the field number and the wire type via the formula `(field_number << 3) | wire_type`.
> In other words, after decoding the varint representing a field, the low 3 bits tell us the wire type, and the rest of the integer tells us the field number.

The wire-type for strings and bytes is the same: 2 = `0x10`.

So the tag of the first field will be (0000 1010)₂ = `0x0A`,
and the tag of the second field will be (0001 0010)₂ = `0x12`.
The length of the address string is 63 bytes = `0x3F`.
The length of the query data is 7 bytes = `0x07`.

|  Hex | Text | Description                |
|------|------|----------------------------|
| `0A` |      | Tag of the first field     |
| `3F` | 63   | Length of the first field  |
| `77` | w    | Address letter             |
| `61` | a    | Address letter             |
| `73` | s    | Address letter             |
| `6D` | m    | Address letter             |
| `31` | 1    | Address letter             |
| `34` | 4    | Address letter             |
| `68` | h    | Address letter             |
| `6A` | j    | Address letter             |
| `32` | 2    | Address letter             |
| `74` | t    | Address letter             |
| `61` | a    | Address letter             |
| `76` | v    | Address letter             |
| `71` | q    | Address letter             |
| `38` | 8    | Address letter             |
| `66` | f    | Address letter             |
| `70` | p    | Address letter             |
| `65` | e    | Address letter             |
| `73` | s    | Address letter             |
| `64` | d    | Address letter             |
| `77` | w    | Address letter             |
| `78` | x    | Address letter             |
| `78` | x    | Address letter             |
| `63` | c    | Address letter             |
| `75` | u    | Address letter             |
| `34` | 4    | Address letter             |
| `34` | 4    | Address letter             |
| `72` | r    | Address letter             |
| `74` | t    | Address letter             |
| `79` | y    | Address letter             |
| `33` | 3    | Address letter             |
| `68` | h    | Address letter             |
| `68` | h    | Address letter             |
| `39` | 9    | Address letter             |
| `30` | 0    | Address letter             |
| `76` | v    | Address letter             |
| `68` | h    | Address letter             |
| `75` | u    | Address letter             |
| `6A` | j    | Address letter             |
| `72` | r    | Address letter             |
| `76` | v    | Address letter             |
| `63` | c    | Address letter             |
| `6D` | m    | Address letter             |
| `73` | s    | Address letter             |
| `74` | t    | Address letter             |
| `6C` | l    | Address letter             |
| `34` | 4    | Address letter             |
| `7A` | z    | Address letter             |
| `72` | r    | Address letter             |
| `33` | 3    | Address letter             |
| `74` | t    | Address letter             |
| `78` | x    | Address letter             |
| `6D` | m    | Address letter             |
| `66` | f    | Address letter             |
| `76` | v    | Address letter             |
| `77` | w    | Address letter             |
| `39` | 9    | Address letter             |
| `73` | s    | Address letter             |
| `30` | 0    | Address letter             |
| `70` | p    | Address letter             |
| `68` | h    | Address letter             |
| `67` | g    | Address letter             |
| `34` | 4    | Address letter             |
| `64` | d    | Address letter             |
| `12` |      | Tag of the second field    |
| `07` | 7    | Length of the second field |
| `22` | "    | Query data byte            |
| `76` | v    | Query data byte            |
| `61` | a    | Query data byte            |
| `6C` | l    | Query data byte            |
| `75` | u    | Query data byte            |
| `65` | e    | Query data byte            |
| `22` | "    | Query data byte            |

So the serialized protobuf in HEX is:

```text
0A3F7761736D3134686A32746176713866706573647778786375343472747933686839307668756A7276636D73746C347A723374786D667677397330706867346412072276616C756522
```

### Deserialization

Returned value was: `Cgx7InZhbHVlIjo1M30=` as Base64 encoded bytes.

```shell
$ echo -n "Cgx7InZhbHVlIjo1M30=" | base64 -d | xxd
```

In hex: `0a0c7b2276616c7565223a35337d`.

|  Hex | Text | Description                     |
|------|------|---------------------------------|
| `0a` |      | Tag of the first field (string) |
| `0c` |      | Length of the string = 12       |
| `7b` | {    | Letter of the response          |
| `22` | "    | Letter of the response          |
| `76` | v    | Letter of the response          |
| `61` | a    | Letter of the response          |
| `6c` | l    | Letter of the response          |
| `75` | u    | Letter of the response          |
| `65` | e    | Letter of the response          |
| `22` | "    | Letter of the response          |
| `3a` | :    | Letter of the response          |
| `35` | 5    | Letter of the response          |
| `33` | 3    | Letter of the response          |
| `7d` | }    | Letter of the response          |

So the response is JSON string: `{"value":53}`.

