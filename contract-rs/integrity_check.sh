#!/bin/sh

# Define color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo "----------------Checking platform balance ----------------"
USDT_BALANCE=$(near contract call-function as-read-only usdt.fakes.testnet ft_balance_of json-args '{"account_id": "decorous-effect.testnet"}' network-config testnet now 2>/dev/null)

USDT_BALANCE=$(echo "$USDT_BALANCE" | bc)

USDT_BALANCE=$(echo "$USDT_BALANCE / 1000000" | bc -l)

echo "Platform USDT balance: $USDT_BALANCE"

INVESTORS_DEPOSIT=$(near contract call-function as-read-only decorous-effect.testnet get_total_deposit json-args '{}' network-config testnet now 2>/dev/null)

INVESTORS_DEPOSIT=$(echo "$INVESTORS_DEPOSIT" | bc) 

echo "Investors USDT deposit: $INVESTORS_DEPOSIT"

result=$(echo "$USDT_BALANCE >= $INVESTORS_DEPOSIT" | bc)

if [[ $result -eq 1 ]]; then
    echo "${GREEN}All good! Platform balance is greater than or equal to investors deposit.${NC}"
else
    echo "${RED}Integrity check failed! Platform balance is less than investors deposit.${NC}"
fi