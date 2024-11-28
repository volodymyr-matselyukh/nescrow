# Nescrow contract

In this contract `usdt.fakes.testnet` is used as a contract for USDT fungible token.

## Useful commands

# Read USDT contract metadata

`near contract call-function as-read-only usdt.fakes.testnet ft_metadata json-args {} network-config testnet now`

# Read volodymyr_matseliukh1.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read nescrow.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "nescrow.testnet"}' network-config testnet now`

# Read macho-metal.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "macho-metal.testnet"}' network-config testnet now`

# Call ft_transfer register nescrow.testnet. This will deposit 10 USDT for nescrow

`near contract call-function as-transaction usdt.fakes.testnet ft_transfer_call json-args '{"amount": "10", "receiver_id": "macho-metal.testnet", "msg": "{ \"username\": \"nescrow\" }"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as nescrow.testnet network-config testnet sign-with-keychain send`

# Call storage_deposit. This will register the account in usdt contract

`near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "macho-metal.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as macho-metal.testnet network-config testnet sign-with-keychain send`

# Call register_customer register nescrow

`near contract call-function as-transaction macho-metal.testnet register_customer json-args '{"username": "nescrow", "account_id": "nescrow.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as macho-metal.testnet network-config testnet sign-with-keychain send`

# Read get_deposit_by_username

`near contract call-function as-read-only macho-metal.testnet get_deposit_by_username json-args '{"sender_username": "vovik"}' network-config testnet now`

# Read get_withdrawable_amount_by_account

`near contract call-function as-read-only macho-metal.testnet get_withdrawable_amount_by_account json-args '{"sender_username": "vovik"}' network-config testnet now`

# Read get_owners_tasks

`near contract call-function as-read-only macho-metal.testnet get_owner_tasks json-args '{"task_owner": "truthful-circle.testnet"}' network-config testnet now`

# Read get_task

`near contract call-function as-read-only macho-metal.testnet get_task json-args '{"task_id": "aa32f1aa-5016-4461-afb8-c8d754e211a6"}' network-config testnet now`

# Call reset_claim

`near contract call-function as-transaction macho-metal.testnet reset_claim json-args '{"task_id": "e7d34dcd-aedc-4e36-948b-a3f824600e57"}' prepaid-gas '100.0 Tgas' attached-deposit '0 Near' sign-as macho-metal.testnet network-config testnet sign-with-keychain send`
