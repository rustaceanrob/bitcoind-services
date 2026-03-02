# Simple Analysis of Bitcoin Peers

Generate JSON for the peers your node is connected to (make sure `bitcoind` is running).

```bash
sh parse.sh # bitcoin-cli getnodeaddresses 0 | jq '.' > nodes.json
```

Build a barchart grouping on network and services

```shell
cargo run --release
```

Filter on peers with compact block filters

```shell
cargo run --release compact_filters
```
