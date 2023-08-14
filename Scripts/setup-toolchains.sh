#!/bin/zsh

# Check if Homebrew is installed, otherwise install it
if ! command -v brew &> /dev/null; then
    echo "Homebrew not found. Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
else
    echo "Homebrew is already installed."
fi

# Update Homebrew
echo "Updating Homebrew..."
brew update


# Install Rust using Homebrew
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    brew install rustup-init
else
    echo "Rust toolchain is already installed."
fi

# Initialize Rustup and configure the default toolchain
echo "Initializing Rustup and setting the default toolchain..."
rustup-init -y

# Add Rust binaries to PATH
echo "Adding Rust binaries to PATH..."
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify Rust installation
rustc --version
cargo --version

echo "Rust environment setup is complete."


# Check if Homebrew is installed, otherwise install it
if ! command -v protoc &> /dev/null; then
    brew install protobuf
fi
