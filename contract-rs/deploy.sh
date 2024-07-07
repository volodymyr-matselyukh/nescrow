#!/bin/sh

./build.sh

echo ">> Deploying contract"

near contract deploy magenta-soda.testnet use-file ./target/wasm32-unknown-unknown/release/nescrow.wasm without-init-call network-config testnet sign-with-keychain send