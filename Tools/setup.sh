#!/usr/bin/env bash

echo
echo "Install Swift Protocol Buffers Libraries"
echo 
echo " - swift-protobuf"
echo " - grpc-swift"
echo
echo "Press [RETURN] to continue:"
read wait

brew update
brew install swift-protobuf grpc-swift
