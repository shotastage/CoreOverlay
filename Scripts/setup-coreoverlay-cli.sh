#!/usr/bin/env bash

# This script is used to setup the coreoverlay for the CLI
echo "Now under construction..."
git clone https://github.com/shotastage/CoreOverlay.git
cd CoreOverlay
swift build
