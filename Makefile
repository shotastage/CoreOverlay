SHELL=/usr/bin/env bash

# Command Definitions
SWIFT := swift
CARGO ?= cargo


# Build configuration
ENABLE_MAC_CATALYST := 0

.PHONY:
setup:
	$(CARGO) install --force cbindgen
	rustup target add aarch64-apple-ios
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-ios
	rustup target add aarch64-apple-ios-sim
ifeq ($(ENABLE_MAC_CATALYST), 1)
	$(CARGO) +nightly build --release -Z build-std --target x86_64-apple-ios-macabi
	$(CARGO) +nightly build --release -Z build-std --target aarch64-apple-ios-high macabi
	rustup target add x86_64-apple-ios-macabi
	rustup target add aarch64-apple-ios-macabi
endif


.PHONY:
run:
	${SWIFT} run

.PHONY:
generate-proto:
	protoc --swift_out=Sources/Protobuf.Generated/ --swift_opt=Visibility=Public --proto_path=proto/ proto/*.proto


.PHONY:
generate-header:
	cbindgen --lang c --output include/engine.h 


.PHONY:
build-rust:
	$(CARGO) build --release --target aarch64-apple-ios
	$(CARGO) build --release --target x86_64-apple-darwin
	$(CARGO) build --release --target aarch64-apple-darwin
	$(CARGO) build --release --target x86_64-apple-ios
	$(CARGO) build --release --target aarch64-apple-ios-sim

ifeq ($(ENABLE_MAC_CATALYST), 1)
	$(CARGO) build --release --target x86_64-apple-ios-macabi
	$(CARGO) build --release --target aarch64-apple-ios-macabi
endif


.PHONY:
build-lipo:
	lipo -create \
  		target/x86_64-apple-darwin/release/libcoreoverlayengine.a \
  		target/aarch64-apple-darwin/release/libcoreoverlayengine.a \
  		-output libcoreoverlayengine_macos.a

	lipo -create \
  		target/x86_64-apple-ios/release/libcoreoverlayengine.a \
  		target/aarch64-apple-ios-sim/release/libcoreoverlayengine.a \
  		-output libcoreoverlayengine_iossimulator.a
ifeq ($(ENABLE_MAC_CATALYST), 1)
	lipo -create \
  		target/x86_64-apple-ios-macabi/release/libcoreoverlayengine.a \
  		target/aarch64-apple-ios-macabi/release/libcoreoverlayengine.a \
  		-output libcoreoverlayengine_maccatalyst.a
endif


.PHONY:
build-swift:
	${SWIFT} build

.PHONY:
build: generate-header generate-proto build-rust build-swift build-lipo

.PHONY:
clean:
	rm -rf .build/
	rm -rf .swiftpm/xcode/
	rm ./Sources/CWasmer/include/CWasmer_generated.h
	find ./Sources/Protobuf.Generated/ -type f -name "*.swift" -delete
	rm -rf target/

.PHONY:
release:
	${SWIFT} build -c release

.PHONY:
update-deps:
	rm Package.resolved
	xcodebuild -resolvePackageDependencies

SDKROOT=(xcrun --sdk macosx --show-sdk-path)


.PHONY:
format:
	${SWIFT} package plugin --allow-writing-to-package-directory swiftformat .
