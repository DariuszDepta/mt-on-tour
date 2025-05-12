# Running dockerized `wasmd`

> Last update: `2025-05-12`

## Building the Docker image

In [wasmd](https://github.com/CosmWasm/wasmd) repository,
there is a [Dockerfile](https://github.com/CosmWasm/wasmd/blob/main/Dockerfile) shown below:

```dockerfile
# docker build . -t cosmwasm/wasmd:latest
# docker run --rm -it cosmwasm/wasmd:latest /bin/sh

FROM golang:1.23.6-alpine AS go-builder

# this comes from standard alpine nightly file
#  https://github.com/rust-lang/docker-rust-nightly/blob/master/alpine3.12/Dockerfile
# with some changes to support our toolchain, etc
RUN set -eux; apk add --no-cache ca-certificates build-base;

RUN apk add git
# NOTE: add these to run with LEDGER_ENABLED=true
# RUN apk add libusb-dev linux-headers

WORKDIR /code
COPY . /code/
# See https://github.com/CosmWasm/wasmvm/releases
ADD https://github.com/CosmWasm/wasmvm/releases/download/v3.0.0-ibc2.0/libwasmvm_muslc.aarch64.a /lib/libwasmvm_muslc.aarch64.a
ADD https://github.com/CosmWasm/wasmvm/releases/download/v3.0.0-ibc2.0/libwasmvm_muslc.x86_64.a /lib/libwasmvm_muslc.x86_64.a
RUN sha256sum /lib/libwasmvm_muslc.aarch64.a | grep 3bffc027c3467d6535fda10e13767194500208add1321709ebd79d2d507eb561
RUN sha256sum /lib/libwasmvm_muslc.x86_64.a | grep 315605d243c71b2c29543af872fa84224632cfd2096755b34bb1798a0587bc42

# force it to use static lib (from above) not standard libgo_cosmwasm.so file
RUN LEDGER_ENABLED=false BUILD_TAGS=muslc LINK_STATICALLY=true make build
RUN echo "Ensuring binary is statically linked ..." \
  && (file /code/build/wasmd | grep "statically linked")

# --------------------------------------------------------
FROM alpine:3.18

COPY --from=go-builder /code/build/wasmd /usr/bin/wasmd

COPY docker/* /opt/
RUN chmod +x /opt/*.sh

WORKDIR /opt

# rest server
EXPOSE 1317
# tendermint p2p
EXPOSE 26656
# tendermint rpc
EXPOSE 26657

CMD ["/usr/bin/wasmd", "version"]
```

The most recent `wasmd` image can be built using
the following [script](https://github.com/DariuszDepta/mt-on-tour/blob/main/rebuild-docker.sh):

```bash
#!/usr/bin/env bash

# set the image/container name
NAME=wasmd

# set the image version
VERSION=3.0.0-ibc2

# remove Docker container (if exists)
container_id="$(docker container list -af name=$NAME -q)"
if [ -z "$container_id" ]
then
  echo "$NAME container not found, skipping deletion"
else
  echo "$NAME container found, deleting"
  docker rm -f "$container_id"
  echo "$NAME container deleted"
fi

# remove Docker image (if exists)
image_id="$(docker images -f reference=$NAME -q)"
if [ -z "$image_id" ]
then
  echo "$NAME image not found, skipping deletion"
else
  echo "$NAME image found, deleting"
  docker rmi "$image_id"
  echo "$NAME image deleted"
fi

# build a new Docker image
docker build -t "$NAME:$VERSION" .

# start a new Docker container
docker run --rm --name $NAME -d "$NAME:$VERSION"

# display logs from running Docker container
docker logs -f "$NAME"
```

This script can be run multiple times, building the image with name and version
specified at the beginning of this file.

Check if the image was built:

```shell
$ docker images
REPOSITORY                               TAG          IMAGE ID       CREATED         SIZE
wasmd                                    3.0.0-ibc2   abe4f2179d93   2 hours ago     184MB
```

## Running containers with blockchain nodes

Now the first container can be started this way:

```shell
$ docker run --rm -it -p 36657:26657 wasmd:3.0.0-ibc2 /bin/sh
```

> Note that the port `26657` is mapped to `36657` for the first blockchain node.

Initialize **node 1** (inside the running Docker container):

```shell
/opt # ./setup_and_run.sh alice bob
```

Two additional keys will be created, named `alice` and `bob`.

In another terminal create the second container:

```shell
$ docker run --rm -it -p 46657:26657 wasmd:3.0.0-ibc2 /bin/sh
```

> Note that the port `26657` is mapped to `46657` for the second blockchain node.

Initialize **node 2** (inside the running Docker container):

```shell
/opt # ./setup_and_run.sh dave edwin
```

Two additional keys will be created, named `dave` and `edwin`.

The same way other nodes can be created in separate containers.
Remember to map `26657` port to some other non-conflicting port in the host machine.
To be able to use keys of all running nodes, keep the key names unique.

## Preparing node addresses

To be able to interact with all nodes in a simple manner, let's define local variables:

```shell
$ NODE1=tcp://0.0.0.0:36657
$ NODE2=tcp://0.0.0.0:46657
```

> Define separate variable for each node with mapped `26657` port.

## Interact with the nodes

There should be no contact code on node 1:

```shell
$ wasmd query wasm list-code -o json --node $NODE1 | jq
```

There should be no contact code on node 2:

```shell
$ wasmd query wasm list-code -o json --node $NODE2 | jq
```

## Import keys locally from running nodes

Using mnemonic phrases for all accounts created in all nodes, import them locally on the host machine:

```shell
$ wasmd keys add alice --recover
$ wasmd keys add bob --recover
$ wasmd keys add dave --recove
$ wasmd keys add edwin --recove
```

Let's check local keys:

```text
- address: wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty
  name: alice
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Ar508Y5H8NpGAj4HbujOj6HOlrsei0Q+iS3i5e7V2mt9"}'
  type: local
- address: wasm1rq3vc2jgg0ra7jqa5cvqtp8rvdeur20r2ugkrv
  name: bob
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AwarVLw20TxjLIPLXn4aR56TyrZWdJ03flhtV0ctv+qq"}'
  type: local
- address: wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw
  name: dave
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A3x/iaSkJJQcxgDA55qj+Enkf0ao27DOclOQLt8jTXoM"}'
  type: local
- address: wasm13d5nz5kl2j6q2rcdaxj9q0daae3tg4k9k833hr
  name: edwin
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AhrNSSt7PiZpF5MZZgW/5evY9e2OjpFWDUPOgSRxdVIn"}'
  type: local
```

## Storing a contract on multiple nodes

> [!NOTE]  
> Assume we have a WASM contract `counter.wasm` already built.

### Use the `counter` contract on the first node

Store the contract:

```shell
$ wasmd tx wasm store counter.wasm --from alice --chain-id testing --gas 10000000 --node $NODE1 -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "B788B54EC5DB6BD198A9A93C6FD3C813DD50EF3BFB8E93CEECA28D6AACF930C1",
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

Check the transaction:

```shell
$ wasmd query tx B788B54EC5DB6BD198A9A93C6FD3C813DD50EF3BFB8E93CEECA28D6AACF930C1 --node $NODE1 -o json | jq
```

Output:

```json
{
  "height": "476",
  "txhash": "B788B54EC5DB6BD198A9A93C6FD3C813DD50EF3BFB8E93CEECA28D6AACF930C1",
  "codespace": "",
  "code": 0,
  "data": "124E0A262F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465526573706F6E7365122408011220D978A2F76CE30886E844881C575AEC85599A2029A0683DF0A6DC963B34037B63",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "10000000",
  "gas_used": "1306539",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "body": {
      "messages": [
        {
          "@type": "/cosmwasm.wasm.v1.MsgStoreCode",
          "sender": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
          "wasm_byte_code": "omitted",
          "instantiate_permission": null
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
            "key": "Ar508Y5H8NpGAj4HbujOj6HOlrsei0Q+iS3i5e7V2mt9"
          },
          "mode_info": {
            "single": {
              "mode": "SIGN_MODE_DIRECT"
            }
          },
          "sequence": "0"
        }
      ],
      "fee": {
        "amount": [],
        "gas_limit": "10000000",
        "payer": "",
        "granter": ""
      },
      "tip": null
    },
    "signatures": [
      "PsEgs1o9gnmDok3IdJIVrROuKuIY3he/eqA1vFcDpPUU+Nb/NN49KZ/mpAJHomcUZDwur4YbuZG0Fzx+fBJJqw=="
    ]
  },
  "timestamp": "2025-05-12T14:11:49Z",
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
          "value": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "acc_seq",
          "value": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty/0",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "signature",
          "value": "PsEgs1o9gnmDok3IdJIVrROuKuIY3he/eqA1vFcDpPUU+Nb/NN49KZ/mpAJHomcUZDwur4YbuZG0Fzx+fBJJqw==",
          "index": true
        }
      ]
    },
    {
      "type": "message",
      "attributes": [
        {
          "key": "action",
          "value": "/cosmwasm.wasm.v1.MsgStoreCode",
          "index": true
        },
        {
          "key": "sender",
          "value": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
          "index": true
        },
        {
          "key": "module",
          "value": "wasm",
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
      "type": "store_code",
      "attributes": [
        {
          "key": "code_checksum",
          "value": "d978a2f76ce30886e844881c575aec85599a2029a0683df0a6dc963b34037b63",
          "index": true
        },
        {
          "key": "code_id",
          "value": "1",
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

Check if the contract code was properly stored on chain:

```shell
$ wasmd query wasm list-code --node $NODE1 -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
      "data_hash": "D978A2F76CE30886E844881C575AEC85599A2029A0683DF0A6DC963B34037B63",
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

Instantiate a contract with initial value set to 10:

```shell
$ wasmd tx wasm instantiate 1 '{"set":10}' --label my-counter-1 --no-admin --from alice --chain-id testing --node $NODE1 -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "47C91499010100A207872407DC24F5E3715979ADA17F69EADB5F3BD9430078DC",
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

Check the transaction:

```shell
$ wasmd query tx 47C91499010100A207872407DC24F5E3715979ADA17F69EADB5F3BD9430078DC --node $NODE1 -o json | jq
```

Output:

```json
{
  "height": "554",
  "txhash": "47C91499010100A207872407DC24F5E3715979ADA17F69EADB5F3BD9430078DC",
  "codespace": "",
  "code": 0,
  "data": "12750A302F636F736D7761736D2E7761736D2E76312E4D7367496E7374616E7469617465436F6E7472616374526573706F6E736512410A3F7761736D3134686A32746176713866706573647778786375343472747933686839307668756A7276636D73746C347A723374786D6676773973307068673464",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "200000",
  "gas_used": "137270",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "body": {
      "messages": [
        {
          "@type": "/cosmwasm.wasm.v1.MsgInstantiateContract",
          "sender": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
          "admin": "",
          "code_id": "1",
          "label": "my-counter-1",
          "msg": {
            "set": 10
          },
          "funds": []
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
            "key": "Ar508Y5H8NpGAj4HbujOj6HOlrsei0Q+iS3i5e7V2mt9"
          },
          "mode_info": {
            "single": {
              "mode": "SIGN_MODE_DIRECT"
            }
          },
          "sequence": "1"
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
      "DaAbUIW+FRs5AYmoiJENQdKH4KP/T7r8fSBA0YCHC9dtGCqOJpuqf13vql0ZTXgHDNY1PJJicFrqQ19GgGCw1w=="
    ]
  },
  "timestamp": "2025-05-12T14:18:22Z",
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
          "value": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "acc_seq",
          "value": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty/1",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "signature",
          "value": "DaAbUIW+FRs5AYmoiJENQdKH4KP/T7r8fSBA0YCHC9dtGCqOJpuqf13vql0ZTXgHDNY1PJJicFrqQ19GgGCw1w==",
          "index": true
        }
      ]
    },
    {
      "type": "message",
      "attributes": [
        {
          "key": "action",
          "value": "/cosmwasm.wasm.v1.MsgInstantiateContract",
          "index": true
        },
        {
          "key": "sender",
          "value": "wasm1z8wt5d4r925vjxy23fufg6ru9jlj0ncphnhkty",
          "index": true
        },
        {
          "key": "module",
          "value": "wasm",
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
      "type": "instantiate",
      "attributes": [
        {
          "key": "_contract_address",
          "value": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
          "index": true
        },
        {
          "key": "code_id",
          "value": "1",
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

Check if the contract was properly instantiated:

```shell
$ wasmd query wasm list-contract-by-code 1 --node $NODE1 -o json | jq
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

Query the current value of the counter contract (should be 10):

```shell
$ wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' --node $NODE1 -o json | jq
```

Output:

```json
{
  "data": {
    "value": 10
  }
}
```

Ok, all set!

### Use the `counter` contract on the second node

Store the contract:

```shell
$ wasmd tx wasm store counter.wasm --from dave --chain-id testing --gas 10000000 --node $NODE2 -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "8E90722991B30A5E821EBB13907BA576AE5C7DF4E237D3282245AE0BC665A32E",
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

Check the transaction:

```shell
$ wasmd query tx 8E90722991B30A5E821EBB13907BA576AE5C7DF4E237D3282245AE0BC665A32E --node $NODE2 -o json | jq
```

Output:

```json
{
  "height": "547",
  "txhash": "8E90722991B30A5E821EBB13907BA576AE5C7DF4E237D3282245AE0BC665A32E",
  "codespace": "",
  "code": 0,
  "data": "124E0A262F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465526573706F6E7365122408011220D978A2F76CE30886E844881C575AEC85599A2029A0683DF0A6DC963B34037B63",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "10000000",
  "gas_used": "1306539",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "body": {
      "messages": [
        {
          "@type": "/cosmwasm.wasm.v1.MsgStoreCode",
          "sender": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
          "wasm_byte_code": "omitted",
          "instantiate_permission": null
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
            "key": "A3x/iaSkJJQcxgDA55qj+Enkf0ao27DOclOQLt8jTXoM"
          },
          "mode_info": {
            "single": {
              "mode": "SIGN_MODE_DIRECT"
            }
          },
          "sequence": "0"
        }
      ],
      "fee": {
        "amount": [],
        "gas_limit": "10000000",
        "payer": "",
        "granter": ""
      },
      "tip": null
    },
    "signatures": [
      "Nb9cQkDINVhfhHgcdInt9mnptnHDFunreS769oSXk3AaZjO0PeuTlCabtNDSnch3C49v66MU8hAMnYNVkcE5pw=="
    ]
  },
  "timestamp": "2025-05-12T14:23:08Z",
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
          "value": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "acc_seq",
          "value": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw/0",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "signature",
          "value": "Nb9cQkDINVhfhHgcdInt9mnptnHDFunreS769oSXk3AaZjO0PeuTlCabtNDSnch3C49v66MU8hAMnYNVkcE5pw==",
          "index": true
        }
      ]
    },
    {
      "type": "message",
      "attributes": [
        {
          "key": "action",
          "value": "/cosmwasm.wasm.v1.MsgStoreCode",
          "index": true
        },
        {
          "key": "sender",
          "value": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
          "index": true
        },
        {
          "key": "module",
          "value": "wasm",
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
      "type": "store_code",
      "attributes": [
        {
          "key": "code_checksum",
          "value": "d978a2f76ce30886e844881c575aec85599a2029a0683df0a6dc963b34037b63",
          "index": true
        },
        {
          "key": "code_id",
          "value": "1",
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

Check if the contract code was properly stored on chain:

```shell
$ wasmd query wasm list-code --node $NODE2 -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
      "data_hash": "D978A2F76CE30886E844881C575AEC85599A2029A0683DF0A6DC963B34037B63",
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

Instantiate a contract with initial value set to 20:

```shell
$ wasmd tx wasm instantiate 1 '{"set":20}' --label my-counter-1 --no-admin --from dave --chain-id testing --node $NODE2 -o json -y | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "CCA61EB07ACDAA2166047768498F9B883A180D772AABC919FEDF4DAB0A800B96",
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

Check the transaction:

```shell
$ wasmd query tx CCA61EB07ACDAA2166047768498F9B883A180D772AABC919FEDF4DAB0A800B96 --node $NODE2 -o json | jq
```

Output:

```json
{
  "height": "571",
  "txhash": "CCA61EB07ACDAA2166047768498F9B883A180D772AABC919FEDF4DAB0A800B96",
  "codespace": "",
  "code": 0,
  "data": "12750A302F636F736D7761736D2E7761736D2E76312E4D7367496E7374616E7469617465436F6E7472616374526573706F6E736512410A3F7761736D3134686A32746176713866706573647778786375343472747933686839307668756A7276636D73746C347A723374786D6676773973307068673464",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "200000",
  "gas_used": "137270",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "body": {
      "messages": [
        {
          "@type": "/cosmwasm.wasm.v1.MsgInstantiateContract",
          "sender": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
          "admin": "",
          "code_id": "1",
          "label": "my-counter-1",
          "msg": {
            "set": 20
          },
          "funds": []
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
            "key": "A3x/iaSkJJQcxgDA55qj+Enkf0ao27DOclOQLt8jTXoM"
          },
          "mode_info": {
            "single": {
              "mode": "SIGN_MODE_DIRECT"
            }
          },
          "sequence": "1"
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
      "JpJGrE8Q1bSgMeXmCJ5Bhqx9rdG8nVEkMsm4NJZmMYVVKGj0QDp7Z7ckSMgCM06A0y3igXdM4FeXckEiHWDzhg=="
    ]
  },
  "timestamp": "2025-05-12T14:25:09Z",
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
          "value": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "acc_seq",
          "value": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw/1",
          "index": true
        }
      ]
    },
    {
      "type": "tx",
      "attributes": [
        {
          "key": "signature",
          "value": "JpJGrE8Q1bSgMeXmCJ5Bhqx9rdG8nVEkMsm4NJZmMYVVKGj0QDp7Z7ckSMgCM06A0y3igXdM4FeXckEiHWDzhg==",
          "index": true
        }
      ]
    },
    {
      "type": "message",
      "attributes": [
        {
          "key": "action",
          "value": "/cosmwasm.wasm.v1.MsgInstantiateContract",
          "index": true
        },
        {
          "key": "sender",
          "value": "wasm1dx6plv66g9hcgrx8yz53vke4ae0f880vjmu7jw",
          "index": true
        },
        {
          "key": "module",
          "value": "wasm",
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
      "type": "instantiate",
      "attributes": [
        {
          "key": "_contract_address",
          "value": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
          "index": true
        },
        {
          "key": "code_id",
          "value": "1",
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

Check if the contract was properly instantiated:

```shell
$ wasmd query wasm list-contract-by-code 1 --node $NODE2 -o json | jq
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

Query the current value of the counter contract (should be 20):

```shell
$ wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' --node $NODE2 -o json | jq
```

Output:

```json
{
  "data": {
    "value": 20
  }
}
```

Ok, all set!
