# Counter smart contract

Store the contract code on chain:

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/counter.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
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
      "creator": "wasm1fkh6p5vwau79mzqs2qssd0wse72ltk2l8a0u3m",
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

Instantiate a contract with initial value set to 0: 

```shell
$ wasmd tx wasm instantiate 1 '"zero"' --label my-counter-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
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

Set the value of the counter to 53:

```shell
$ wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"set":53}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
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

```shell
$ curl -X GET 'http://0.0.0.0:26657/abci_query?path="/wasm/contract/wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"&data=xxxx'
```

```shell
$ grpcurl -plaintext -d '{"address":"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d","query_data":"InZhbHVlIg=="}' localhost:9090 cosmwasm.wasm.v1.Query/SmartContractState
```

Output:

```json
{
  "data": "eyJ2YWx1ZSI6NTN9"
}
```

```shell
$ curl -X GET 'http://0.0.0.0:26657/abci_query?path="/cosmwasm.wasm.v1.Query/SmartContractState"' -d @data.json
```

```shell
$ curl -s -X GET 'http://0.0.0.0:26657/abci_query?path="/cosmwasm.wasm.v1.Query/Codes"' | jq
```

```
message QuerySmartContractStateRequest {
  string address = 1;
  bytes query_data = 2;
}
```

```text
address = wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d
bytes = InZhbHVlIg==
```

```text
Cj93YXNtMTRoajJ0YXZxOGZwZXNkd3h4Y3U0NHJ0eTNoaDkwdmh1anJ2Y21zdGw0enIzdHhtZnZ3OXMwcGhnNGQSDEluWmhiSFZsSWc9PQ==
```

```text
$ curl -s -X GET 'http://0.0.0.0:26657/abci_query?path="/cosmwasm.wasm.v1.Query/SmartContractState"&data="Cj93YXNtMTRoajJ0YXZxOGZwZXNkd3h4Y3U0NHJ0eTNoaDkwdmh1anJ2Y21zdGw0enIzdHhtZnZ3OXMwcGhnNGQSByJ2YWx1ZSI="'
$ curl -s -X GET 'http://0.0.0.0:26657/abci_query?path="/cosmwasm.wasm.v1.Query/SmartContractState"&data=""'
```

This text:
```text
{"address":"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d","query_data":"InZhbHVlIg=="}
```

in base64:
```text
eyJhZGRyZXNzIjoid2FzbTE0aGoydGF2cThmcGVzZHd4eGN1NDRydHkzaGg5MHZodWpydmNtc3RsNHpyM3R4bWZ2dzlzMHBoZzRkIiwicXVlcnlfZGF0YSI6IkluWmhiSFZsSWc9PSJ9
```

