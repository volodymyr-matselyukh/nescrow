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
near contract call-function as-transaction usdt.fakes.testnet ft_transfer json-args '{"amount": "2", "receiver_id": "scandalous-eggnog.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send
`

# Call storage_deposit
`
near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "scandalous-eggnog.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as scandalous-eggnog.testnet network-config testnet sign-with-keychain send
`