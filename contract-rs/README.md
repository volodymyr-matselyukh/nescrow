# Nescrow contract

In this contract `usdt.fakes.testnet` is used as a contract for USDT fungible token.

## Useful commands

# Read USDT contract metadata
`
near contract call-function as-read-only usdt.fakes.testnet ft_metadata json-args {} network-config testnet now
`

# Read volodymyr_matseliukh1.testnet USDT balance

`
near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "volodymyr_matseliukh1.testnet"}' network-config testnet now
`

# Call ft_transfer
`
near contract call-function as-transaction usdt.fakes.testnet ft_transfer_call json-args '{"amount": "2", "receiver_id": "scandalous-eggnog.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send
`

`
near contract call-function as-transaction usdt.fakes.testnet ft_transfer_call json-args '{"amount": "2", "msg":"", "receiver_id": "scandalous-eggnog.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as evasive-dime.testnet network-config testnet sign-with-keychain send
`

# Call storage_deposit. This will register the account in usdt contract
`
near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "volodymyr_matseliukh2.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as volodymyr_matseliukh2.testnet network-config testnet sign-with-keychain send
`

# Call register_customer
`
near contract call-function as-transaction scandalous-eggnog.testnet register_customer json-args '{"customer_id": "witty-activity.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as witty-activity.testnet network-config testnet sign-with-keychain send
`
