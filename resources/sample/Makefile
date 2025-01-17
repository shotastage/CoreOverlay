# OS detection
UNAME_S := $(shell uname -s)

.PHONY:
setup:
ifeq ($(UNAME_S),Darwin)  # Darwin is macOS
	brew update
else
	@echo "Skipping Homebrew commands (not on macOS)"
endif
	rustup target add wasm32-wasip1


.PHONY:
build:
	@echo "Building sample-rust package..."
	cd sample-rust && cargo build --release --target wasm32-wasip1

.PHONY:
test-all:
	wasmer ../../target/wasm32-wasip1/release/sample-rust.wasm

.PHONY:
clean:
	@rm hello-rust.wasm
#	rm hello-rust.lvmpc
	@echo "Cleaning sample-rust package..."
	cd sample-rust && cargo clean
