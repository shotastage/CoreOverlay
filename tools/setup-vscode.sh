#!/usr/bin/env bash

if command -v code >/dev/null 2>&1; then
    echo "Visual Studio Code is installed."
else
    echo "Visual Studio Code is not installed."
    echo "Please install Visual Studio Code from https://code.visualstudio.com/."
    exit 1
fi

code --install-extension rust-lang.rust-analyzer
code --install-extension juggernautjp.less-toml
code --install-extension editorconfig.editorconfig
