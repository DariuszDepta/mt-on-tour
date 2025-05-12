# Running dockerized `wasmd`

```shell
$ docker run --rm -it -p 36657:26657 wasmd:3.0.0-ibc2 /bin/sh
/opt # ./setup_and_run.sh
```

```shell
$ docker run --rm -it -p 46657:26657 wasmd:3.0.0-ibc2 /bin/sh
/opt # ./setup_and_run.sh
```

```shell
$ wasmd query wasm list-code -o json --node "tcp://0.0.0.0:36657" | jq
$ NODE1=tcp://0.0.0.0:36657
$ wasmd query wasm list-code -o json --node $NODE1 | jq
```

```shell
$ wasmd query wasm list-code -o json --node "tcp://0.0.0.0:46657" | jq
$ NODE2=tcp://0.0.0.0:46657
$ wasmd query wasm list-code -o json --node $NODE2 | jq
```
