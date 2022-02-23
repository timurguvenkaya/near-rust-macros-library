#!/bin/bash
rm -rf ~/.near-credentials/testnet/dev-*

ACCOUNT=$(near dev-deploy | sed -n '5,1p' | grep -o -E "dev-\d+-\d+")

# Initialize
near call $ACCOUNT new '{"owner": "timurguvenkaya.testnet", "minter": "app.timurguvenkaya.testnet", "manager": "app.timurguvenkaya.testnet", "data": "SOME_DATA"}' --accountId $ACCOUNT

# Check whether private function is not accessible
near call $ACCOUNT setup_account_role '{"role":"minter", "account":"manager.near"}' --accountId timurguvenkaya.testnet

# Check whether private function is not accessible
near call $ACCOUNT add_role_member '{"role":"minter", "account":"timurguvenkaya.near"}' --accountId timurguvenkaya.testnet

# Check whether private function is not accessible
near call $ACCOUNT add_role '{"role":"pauser"}' --accountId timurguvenkaya.testnet

# Set admin manager to be the admin role of Minters
near call $ACCOUNT set_admin_role '{"role":"minter", "admin_role":"manager"}' --accountId timurguvenkaya.testnet

# Should fail because timurguvenkaya.testnet does not have a "manager" role
near call $ACCOUNT grant_role '{"role": "minter", "account": "timurguvenkaya.testnet"}' --accountId timurguvenkaya.testnet

# Get data before pause
near view $ACCOUNT get_data --accountId timurguvenkaya.testnet

# Should fail because app.timurguvenkaya.testnet does not have "default_admin" role
near call $ACCOUNT pub_toggle_pause '{}' --accountId app.timurguvenkaya.testnet

# Toggle pause
near call $ACCOUNT pub_toggle_pause '{}' --accountId timurguvenkaya.testnet

# Should fail because paused
near view $ACCOUNT get_data --accountId timurguvenkaya.testnet

# Toggle pause (unpause)
near call $ACCOUNT pub_toggle_pause '{}' --accountId timurguvenkaya.testnet

# Get data after pause
near view $ACCOUNT get_data --accountId timurguvenkaya.testnet
