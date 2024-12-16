#!/bin/sh

OLD_ACCOUNT="${1:-nothing}"
NEW_ACCOUNT="${2:-nothing}"

if [[ "$OLD_ACCOUNT" == "nothing" ]]; then
    echo "Specify old account:"
    read OLD_ACCOUNT
fi

if [[ "$NEW_ACCOUNT" == "nothing" ]]; then
    echo "Specify new account:"
    read NEW_ACCOUNT
fi

echo "----------------Moving funds from $OLD_ACCOUNT to $NEW_ACCOUNT ----------------"

echo "----------------Registering new account in USDT fakes----------------"

near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "'$(echo "$NEW_ACCOUNT")'"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as $OLD_ACCOUNT network-config testnet sign-with-keychain send

echo "----------------Transferring USDT to new account----------------"

USDT_BALANCE=$(near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "uncovered-disease.testnet"}' network-config testnet now)

near contract call-function as-transaction usdt.fakes.testnet ft_transfer json-args '{"amount": '$(echo "$USDT_BALANCE")', "receiver_id": "'$(echo "$NEW_ACCOUNT")'"}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as $OLD_ACCOUNT network-config testnet sign-with-keychain send

echo "----------------Removing old account----------------"

near account delete-account $OLD_ACCOUNT beneficiary $NEW_ACCOUNT network-config testnet sign-with-keychain send