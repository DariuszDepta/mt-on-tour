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

Check if the contract code was stored on chain:

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

Check if the contract is instantiated:
 
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
message QuerySmartContractStateRequest {
  required string address = 1;
  required bytes query_data = 2;
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
message QuerySmartContractStateRequest {
  required string address = 1;
  required bytes query_data = 2;
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

