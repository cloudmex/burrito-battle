#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --all --target wasm32-unknown-unknown --release

if [ ! -d res/ ];
then
mkdir res
fi

cp target/wasm32-unknown-unknown/release/pvp.wasm ./res/

echo "¿Quieres desplegar el contrato de batallas pvp?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near dev-deploy --wasmFile res/pvp.wasm; break;;
        No ) exit;;
    esac
done