#!/bin/bash
rm -rf ~/.near-credentials/testnet/dev-*

ACCOUNT=$(near dev-deploy | sed -n '5,1p' | grep -o -E "dev-\d+-\d+")

near call $ACCOUNT new '{"data": "SOME_DATA"}' --accountId $ACCOUNT

near view $ACCOUNT get_data --accountId $ACCOUNT

near call $ACCOUNT pub_toggle_pause '{}' --accountId $ACCOUNT

near view $ACCOUNT get_data --accountId $ACCOUNT

near call $ACCOUNT pub_toggle_pause '{}' --accountId $ACCOUNT

near view $ACCOUNT get_data --accountId $ACCOUNT
