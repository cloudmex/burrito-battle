#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --all --target wasm32-unknown-unknown --release
rsync -u target/wasm32-unknown-unknown/release/strw_token.wasm res/

set -ex
NETWORK=testnet
OWNER=strw-token.$NETWORK
MASTER_ACC=strw-token.$NETWORK
CONTRACT_ACC=$MASTER_ACC
export NODE_ENV=$NETWORK

echo "¿Quieres desplegar el contrato?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near dev-deploy --wasmFile res/strw_token.wasm; break;;
        No ) exit;;
    esac
done