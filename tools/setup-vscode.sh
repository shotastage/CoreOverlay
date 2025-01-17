#!/usr/bin/env bash

# Check if the VSCode installed
if ! command -v code >/dev/null 2>&1; then
    echo "Visual Studio Code is not installed."
    echo "Please install Visual Studio Code from https://code.visualstudio.com/"
    exit 1
fi

echo "Visual Studio Code is installed."

# List of VSCode extensions
extensions=(
    "rust-lang.rust-analyzer"
    "juggernautjp.less-toml"
    "editorconfig.editorconfig"
)

# Install VSCode extensions
for ext in "${extensions[@]}"; do
    echo "Installing VS Code extension: $ext"
    code --install-extension "$ext"
done

# Install rustfmt
echo "Installing rustfmt..."
rustup component add rustfmt

echo "Setup completed."
