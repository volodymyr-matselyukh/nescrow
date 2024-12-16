# Nescrow contract

In this contract `usdt.fakes.testnet` is used as a contract for USDT fungible token.

## Useful commands

# Read USDT contract metadata

`near contract call-function as-read-only usdt.fakes.testnet ft_metadata json-args {} network-config testnet now`

# Read volodymyr_matseliukh1.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read nescrow.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "nescrow.testnet"}' network-config testnet now`

# Read uncovered-disease.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "uncovered-disease.testnet"}' network-config testnet now`

# Call ft_transfer register nescrow.testnet. This will deposit 10 USDT for nescrow on needofescrow platform

`near contract call-function as-transaction usdt.fakes.testnet ft_transfer_call json-args '{"amount": "10", "receiver_id": "uncovered-disease.testnet", "msg": "{ \"username\": \"nescrow\" }"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as nescrow.testnet network-config testnet sign-with-keychain send`

# Call ft_transfer for uncovered-disease.testnet. This will transfer 428 USDT from uncovered-disease.testnet to nescrow.testnet

`near contract call-function as-transaction usdt.fakes.testnet ft_transfer json-args '{"amount": "428000000", "receiver_id": "nescrow.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as uncovered-disease.testnet network-config testnet sign-with-keychain send`

# Call storage_deposit. This will register the account in usdt contract

`near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "uncovered-disease.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as uncovered-disease.testnet network-config testnet sign-with-keychain send`

# Call register_customer register nescrow

`near contract call-function as-transaction uncovered-disease.testnet register_customer json-args '{"username": "nescrow", "account_id": "nescrow.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as uncovered-disease.testnet network-config testnet sign-with-keychain send`

# Read get_deposit_by_username

`near contract call-function as-read-only uncovered-disease.testnet get_deposit_by_username json-args '{"sender_username": "vovik"}' network-config testnet now`

# Read get_withdrawable_amount_by_account

`near contract call-function as-read-only uncovered-disease.testnet get_withdrawable_amount_by_account json-args '{"sender_username": "nescrow", "account_id": "nescrow.testnet"}' network-config testnet now`

# Read get_owners_tasks

`near contract call-function as-read-only uncovered-disease.testnet get_owner_tasks json-args '{"task_owner": "truthful-circle.testnet"}' network-config testnet now`

# Read get_task

`near contract call-function as-read-only uncovered-disease.testnet get_task json-args '{"task_id": "60847862-9533-4318-acd9-c580356ecd6b"}' network-config testnet now`

# Call reset_claim

`near contract call-function as-transaction uncovered-disease.testnet reset_claim json-args '{"task_id": "e7d34dcd-aedc-4e36-948b-a3f824600e57"}' prepaid-gas '100.0 Tgas' attached-deposit '0 Near' sign-as uncovered-disease.testnet network-config testnet sign-with-keychain send`

# Call migrate_state

`near contract call-function as-transaction uncovered-disease.testnet migrate_state json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as uncovered-disease.testnet network-config testnet sign-with-keychain send`
