# Responder contract

## Store contract on chain

Store the `responder` contract code on chain:

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/responder.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Check if the `responder` contract code was properly stored on chain:

```shell
$ wasmd query wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
      "data_hash": "8D588054FB55844E71B9122657C14FBBE1E13F773787A157C08F443EFD1021F2",
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

Instantiate a `responder` contract: 

```shell
$ wasmd tx wasm instantiate 1 '"none"' --label my-responder-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Check if the `responder` contract was properly instantiated:
 
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

