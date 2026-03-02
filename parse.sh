#!/usr/bin/env bash
bitcoin-cli getnodeaddresses 0 | jq '.' > nodes.json

