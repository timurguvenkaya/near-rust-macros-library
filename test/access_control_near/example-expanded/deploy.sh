#!/bin/bash
rm -rf ~/.near-credentials/testnet/dev-*

ACCOUNT=$(near dev-deploy | sed -n '5,1p' | grep -o -E "dev-\d+-\d+")

near call $ACCOUNT new '{"owner": "timurguvenkaya.testnet", "minter": "app.timurguvenkaya.testnet", "manager": "app.timurguvenkaya.testnet"}' --accountId $ACCOUNT

