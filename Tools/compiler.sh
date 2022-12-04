#!/usr/bin/env bash

# Change Swift toolchain to Wasm supported version
swiftenv global wasm-5.7.1

swiftc -target wasm32-unknown-wasi $1 -o ${1%.*}.wasm
wasm2wat ${1%.*}.wasm -o ${1%.*}.wat
echo $(wat2wasm -o >(base64) ${1%.*}.wat) > ${1%.*}.wab1

mv ${1%.*}.wab1 ./

# Restore defualt version toolchain
swiftenv global system

# Cleaning
rm ${1%.*}.wasm
rm ${1%.*}.wat
