SHELL=/usr/bin/env bash

# Build configuration
ENABLE_MAC_CATALYST := 0

# Command Definitions
SWIFT := swift
CARGO ?= cargo

SDKROOT=(xcrun --sdk macosx --show-sdk-path)



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
setup-ci:
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
pre-build-preparation:
	@echo "Preparing build..."
	@rm -rf ./artifacts/
	@echo "Done!"

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
	mkdir -p ./artifacts
	lipo -create \
  		target/x86_64-apple-darwin/release/libcoreoverlayengine.a \
  		target/aarch64-apple-darwin/release/libcoreoverlayengine.a \
  		-output ./artifacts/libcoreoverlayengine_macos.a

	lipo -create \
  		target/x86_64-apple-ios/release/libcoreoverlayengine.a \
  		target/aarch64-apple-ios-sim/release/libcoreoverlayengine.a \
  		-output ./artifacts/libcoreoverlayengine_iossimulator.a
ifeq ($(ENABLE_MAC_CATALYST), 1)
	lipo -create \
  		target/x86_64-apple-ios-macabi/release/libcoreoverlayengine.a \
  		target/aarch64-apple-ios-macabi/release/libcoreoverlayengine.a \
  		-output ./artifacts/libcoreoverlayengine_maccatalyst.a
endif


.PHONY:
build-rust-framework:
	xcodebuild -create-xcframework \
  		-library ./artifacts/libcoreoverlayengine_macos.a -headers ./include/ \
  		-library ./artifacts/libcoreoverlayengine_iossimulator.a -headers ./include/ \
  		-library ./target/aarch64-apple-ios/release/libcoreoverlayengine.a -headers ./include/ \
  		-output ./artifacts/CoreOverlayEngine.xcframework

ifeq ($(ENABLE_MAC_CATALYST), 1)
	xcodebuild -create-xcframework \
  		-library ./artifacts/libcoreoverlayengine_macos.a -headers ./include/ \
  		-library ./artifacts/libcoreoverlayengine_iossimulator.a -headers ./include/ \
  		-library ./artifacts/libcoreoverlayengine_maccatalyst.a -headers ./include/ \
  		-library ./target/aarch64-apple-ios/release/libcoreoverlayengine.a -headers ./include/ \
  		-output ./artifacts/CoreOverlayEngine.xcframework
endif


.PHONY:
build-swift:
	${SWIFT} build


.PHONY:
build-artifacts:
	@echo "Zipping artifacts..."
	@zip -r ./artifacts/bundle.zip ./artifacts/CoreOverlayEngine.xcframework

.PHONY:
build-finalize:
	@echo "Cleaning up..."
	@echo
	@rm ./artifacts/libcoreoverlayengine_macos.a
	@rm ./artifacts/libcoreoverlayengine_iossimulator.a
	@echo "Build complete!"
	@echo "Information:"
	@echo "Artifacts are located in ./artifacts"
	@openssl dgst -sha256 ./artifacts/bundle.zip

.PHONY:
build: pre-build-preparation generate-header generate-proto build-rust build-lipo build-rust-framework build-artifacts build-swift build-finalize


.PHONY:
build-ci: pre-build-preparation build-rust build-lipo build-rust-framework build-artifacts build-swift build-finalize
	rm -rf ./artifacts/CoreOverlayEngine.xcframework

.PHONY:
clean:
	@echo "Cleaning project..."
	@rm -rf .build/
	@rm -rf .swiftpm/xcode/
	@find ./Sources/Protobuf.Generated/ -type f -name "*.swift" -delete
	@rm -rf target/
	@rm -rf ./artifacts/
	@rm Cargo.lock
	@echo "Done!"

.PHONY:
release:
	${SWIFT} build -c release

.PHONY:
update-deps:
	@rm Package.resolved
	xcodebuild -resolvePackageDependencies

.PHONY:
format:
	${SWIFT} package plugin --allow-writing-to-package-directory swiftformat .
