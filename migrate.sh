#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --all --target wasm32-unknown-unknown --release

if [ ! -d res/ ];
then
mkdir res
fi

cp target/wasm32-unknown-unknown/release/nft_aucions.wasm ./res/

echo "¿Quieres migrar el contrato de auction?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near deploy  --wasmFile res/nft_aucions.wasm --initFunction "cleanup"  --initArgs "{}"  --accountId $CONTRACT_ID; break;;
        No ) exit;;
    esac
done