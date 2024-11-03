# Nescrow contract

In this contract `usdt.fakes.testnet` is used as a contract for USDT fungible token.

## Useful commands

# Read USDT contract metadata

`near contract call-function as-read-only usdt.fakes.testnet ft_metadata json-args {} network-config testnet now`

# Read volodymyr_matseliukh1.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read nescrow.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "nescrow.testnet"}' network-config testnet now`

# Read ad-brick.testnet USDT balance

`near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "ad-brick.testnet"}' network-config testnet now`

# Call ft_transfer

`near contract call-function as-transaction usdt.fakes.testnet ft_transfer_call json-args '{"amount": "2", "receiver_id": "ad-brick.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send`

`near contract call-function as-transaction usdt.fakes.testnet ft_transfer_call json-args '{"amount": "2", "msg":"", "receiver_id": "ad-brick.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as evasive-dime.testnet network-config testnet sign-with-keychain send`

# Call storage_deposit. This will register the account in usdt contract

`near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "ad-brick.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as ad-brick.testnet network-config testnet sign-with-keychain send`

# Call register_customer

`near contract call-function as-transaction ad-brick.testnet register_customer json-args '{"username": "witty"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as ad-brick.testnet network-config testnet sign-with-keychain send`

# Read get_withdrawable_amount

`near contract call-function as-read-only ad-brick.testnet get_withdrawable_amount json-args '{"sender_email": "vova@navirego.com", "account_id": "truthful-circle.testnet"}' network-config testnet now`

# Read get_owners_tasks

`near contract call-function as-read-only ad-brick.testnet get_owner_tasks json-args '{"task_owner": "truthful-circle.testnet"}' network-config testnet now`

# Read get_task

`near contract call-function as-read-only ad-brick.testnet get_task json-args '{"task_id": "e7d34dcd-aedc-4e36-948b-a3f824600e57"}' network-config testnet now`

# Call reset_claim

`near contract call-function as-transaction ad-brick.testnet reset_claim json-args '{"task_id": "e7d34dcd-aedc-4e36-948b-a3f824600e57"}' prepaid-gas '100.0 Tgas' attached-deposit '0 Near' sign-as ad-brick.testnet network-config testnet sign-with-keychain send`