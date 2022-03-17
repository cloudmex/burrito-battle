#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/

echo "¿Quieres desplegar el contrato?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near dev-deploy --wasmFile res/non_fungible_token.wasm; break;;
        No ) exit;;
    esac
done