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

Send some tokens from `alice` to instantiated smart contract:

```shell
$ wasmd tx bank send alice wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d 100stake --from=alice --chain-id wte --keyring-backend=test -o json -y | jq  
```

Output:

```json
{
  "height": "0",
  "txhash": "A26E21C749EB95F65B979D0258326039CAC582CCC0AFF0899E04BF6A8C476E9F",
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

Let's check the transaction details:

```shell
$ wasmd q tx A26E21C749EB95F65B979D0258326039CAC582CCC0AFF0899E04BF6A8C476E9F -o json | jq
```

Output:

```json
{
  "height": "195",
  "txhash": "A26E21C749EB95F65B979D0258326039CAC582CCC0AFF0899E04BF6A8C476E9F",
  "codespace": "",
  "code": 0,
  "data": "12260A242F636F736D6F732E62616E6B2E763162657461312E4D736753656E64526573706F6E7365",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "200000",
  "gas_used": "53654",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "body": {
      "messages": [
        {
          "@type": "/cosmos.bank.v1beta1.MsgSend",
          "from_address": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
          "to_address": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
          "amount": [
            {
              "denom": "stake",
              "amount": "100"
            }
          ]
        }
      ],
      "memo": "",
      "timeout_height": "0",
      "extension_options": [],
      "non_critical_extension_options": []
    },
    "auth_info": {
      "signer_infos": [
        {
          "public_key": {
            "@type": "/cosmos.crypto.secp256k1.PubKey",
            "key": "AglVqKaCCgzIYtWv1F8zdA83+2EQPaxnkkBHP+WGTaJE"
          },
          "mode_info": {
            "single": {
              "mode": "SIGN_MODE_DIRECT"
            }
          },
          "sequence": "3"
        }
      ],
      "fee": {
        "amount": [],
        "gas_limit": "200000",
        "payer": "",
        "granter": ""
      },
      "tip": null
    },
    "signatures": [
      "aSqPY25Bxq5oRY5dmBwfwG2QO5k/X6dXMDS/PKIEw5J4f/cSrMa3udKBHXyDomLXJtCUbJYxBW2izjofZ/tOyQ=="
    ]
  },
  "timestamp": "2025-01-20T10:12:43Z",
  "events": [
    {
      "type": "tx",
      "attributes": [
        {
          "key": "fee",
          "value": "",
          "index": true
        },
        {
          "key": "fee_payer",
          "value": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "acc_seq",
          "value": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam/3",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "signature",
          "value": "aSqPY25Bxq5oRY5dmBwfwG2QO5k/X6dXMDS/PKIEw5J4f/cSrMa3udKBHXyDomLXJtCUbJYxBW2izjofZ/tOyQ==",
          "index": true
        }
      ]
    },
    {
      "type": "message",
      "attributes": [
        {
          "key": "action",
          "value": "/cosmos.bank.v1beta1.MsgSend",
          "index": true
        },
        {
          "key": "sender",
          "value": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
          "index": true
        },
        {
          "key": "module",
          "value": "bank",
          "index": true
        },
        {
          "key": "msg_index",
          "value": "0",
          "index": true
        }
      ]
    },
    {
      "type": "coin_spent",
      "attributes": [
        {
          "key": "spender",
          "value": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
          "index": true
        },
        {
          "key": "amount",
          "value": "100stake",
          "index": true
        },
        {
          "key": "msg_index",
          "value": "0",
          "index": true
        }
      ]
    },
    {
      "type": "coin_received",
      "attributes": [
        {
          "key": "receiver",
          "value": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
          "index": true
        },
        {
          "key": "amount",
          "value": "100stake",
          "index": true
        },
        {
          "key": "msg_index",
          "value": "0",
          "index": true
        }
      ]
    },
    {
      "type": "transfer",
      "attributes": [
        {
          "key": "recipient",
          "value": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
          "index": true
        },
        {
          "key": "sender",
          "value": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
          "index": true
        },
        {
          "key": "amount",
          "value": "100stake",
          "index": true
        },
        {
          "key": "msg_index",
          "value": "0",
          "index": true
        }
      ]
    },
    {
      "type": "message",
      "attributes": [
        {
          "key": "sender",
          "value": "wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam",
          "index": true
        },
        {
          "key": "msg_index",
          "value": "0",
          "index": true
        }
      ]
    }
  ]
}
```

Let's check the balances again, for `alice`:

```shell
$ wasmd query bank balances wasm10pmzqzk5tp4893yk8zkc8km3dlurn94pggywam -o json | jq
```

Output:

```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "999749999900"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

And for instantiated contract:

```shell
$ wasmd query bank balances wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq
```

Output:

```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "100"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

