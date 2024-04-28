#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

#https://docs.near.org/develop/contracts/quickstart
npx near deploy malicious-basketball.testnet build/contract.wasm 