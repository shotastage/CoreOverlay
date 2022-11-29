#!/usr/bin/env bash

echo
echo "Install Swift Protocol Buffers Libraries"
echo 
echo " - swift-protobuf"
echo " - grpc-swift"
echo " - wabt"

echo
echo "Press [RETURN] to continue:"
read wait


check_and_install ()
{
    if type $0 > /dev/null 2>&1; then
        echo $0 command is already installed.
    else
        brew install $1
    fi
}

brew update

# check_and_install protoc swift-protobuf
# check_and_install protoc-gen-swift grpc-swift
# check_and_install wat2wasm wabt
brew install swift-protobuf grpc-swift wabt
