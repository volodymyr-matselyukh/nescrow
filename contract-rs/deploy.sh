#!/bin/sh

./build.sh

echo ">> Deploying contract"

near contract deploy malicious-basketball.testnet use-file ./target/wasm32-unknown-unknown/release/guestbook.wasm without-init-call network-config testnet sign-with-keychain send