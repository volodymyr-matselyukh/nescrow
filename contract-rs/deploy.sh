#!/bin/sh

./build.sh

echo ">> Deploying contract"

near contract deploy scandalous-eggnog.testnet use-file ./target/wasm32-unknown-unknown/release/nescrow.wasm without-init-call network-config testnet sign-with-keychain send