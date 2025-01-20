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

## Check balances

List all keys:

```shell
$ wasmd keys list --keyring-backend=test --output json | jq
```

Output:

```json
[
  {
    "name": "alice",
    "type": "local",
    "address": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
    "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AglVqKaCCgzIYtWv1F8zdA83+2EQPaxnkkBHP+WGTaJE\"}"
  },
  {
    "name": "bob",
    "type": "local",
    "address": "wasm1l2k354gfve93rrjhcnvkaa6f50tucctgchfgmf",
    "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A648j/lfQdPh35IEXkzky/Wo071pSTed5ESomFgEI6RT\"}"
  },
  {
    "name": "cecil",
    "type": "local",
    "address": "wasm1dfzjx4n2j6w99zpadw80c37cfv3utncdfu4a8s",
    "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"Al7zvtEluFMXofaGtj46zDn8TJWUm+NMBHAjIKSnzCfJ\"}"
  },
  {
    "name": "dave",
    "type": "local",
    "address": "wasm1j26ltgh0aazkmlnc7evaz2wkku65qk7ep63gyg",
    "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"Aru/864qqE94MdiWf9VKPrM9GZY33Ag6Mw4I0u/1yyj6\"}"
  }
]
```

Check the balances for `alice`:

```shell
$ wasmd query bank balances wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam -o json | jq
```

Output:

```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "999750000000"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

Check the balances for instantiated contract:

```shell
$ wasmd query bank balances wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq
```

Output:

```json
{
  "balances": [],
  "pagination": {}
}
```

