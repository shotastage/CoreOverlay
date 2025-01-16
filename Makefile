SHELL=/usr/bin/env bash

# Build configuration
ENABLE_MAC_CATALYST := 0
ENABLE_ANDROID_SUPPORT := 0

# Command Definitions
SWIFT := swift
CARGO ?= cargo
GO ?= go
SDKROOT=(xcrun --sdk macosx --show-sdk-path)


###############################################################
## BUILD ENV SETUP FOR DEVELOPMENT                           ##
###############################################################
.PHONY:
setup:
	$(CARGO) install --force cbindgen
	rustup target add aarch64-apple-ios
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-ios
	rustup target add aarch64-apple-ios-sim
ifeq ($(ENABLE_ANDROID_SUPPORT), 1)
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add i686-linux-android
	rustup target add x86_64-linux-android
endif
ifeq ($(ENABLE_MAC_CATALYST), 1)
	$(CARGO) +nightly build --release -Z build-std --target x86_64-apple-ios-macabi
	$(CARGO) +nightly build --release -Z build-std --target aarch64-apple-ios-high macabi
	rustup target add x86_64-apple-ios-macabi
	rustup target add aarch64-apple-ios-macabi
endif


###############################################################
## BUILD ENV SETUP FOR CI                                    ##
###############################################################
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


###############################################################
## BUILD PROCEDURES                                          ##
###############################################################

## Preparation for building
.PHONY:
pre-build-preparation:
	@echo "Preparing build..."
	@rm -rf ./artifacts/

## Rust build procedure
.PHONY:
build-main-components:
    $(CARGO) build --release --target aarch64-apple-ios
    $(CARGO) build --release --target x86_64-apple-darwin
    $(CARGO) build --release --target aarch64-apple-darwin
    $(CARGO) build --release --target x86_64-apple-ios
    $(CARGO) build --release --target aarch64-apple-ios-sim

.PHONY:
build-apple-platform-framework:
    $(CARGO) build --release --target aarch64-apple-ios
	$(CARGO) build --release --target x86_64-apple-darwin
	$(CARGO) build --release --target aarch64-apple-darwin
	$(CARGO) build --release --target x86_64-apple-ios
	$(CARGO) build --release --target aarch64-apple-ios-sim
ifeq ($(ENABLE_MAC_CATALYST), 1)
	$(CARGO) build --release --target x86_64-apple-ios-macabi
	$(CARGO) build --release --target aarch64-apple-ios-macabi
endif


## Generate Universal Binary
.PHONY:
build-universal-bin:
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


## Build Rust package and make as XCFramework
.PHONY:
build-rust-xcframework:
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


## Build Swift package
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


###############################################################
## GATHERED ALL BUILD PROCEDURE                              ##
###############################################################
.PHONY:
build: pre-build-preparation build-main-components build-universal-bin build-rust-xcframework build-artifacts build-swift build-finalize


###############################################################
## GATHERED ALL BUILD PROCEDURE FOR RELEASE                  ##
###############################################################
.PHONY:
release:
	${SWIFT} build -c release


###############################################################
## GATHERED ALL BUILD PROCEDURE FOR CI                       ##
###############################################################
.PHONY:
build-ci: pre-build-preparation build-rust build-universal-bin build-rust-xcframework build-artifacts build-swift build-finalize
	rm -rf ./artifacts/CoreOverlayEngine.xcframework


###############################################################
## CELAN PROJECT WORKSPACE                                   ##
###############################################################
.PHONY:
clean:
	@echo "Cleaning project..."
	@rm -rf .build/
	@rm -rf .swiftpm/xcode/
	@cargo clean
	@rm -rf ./artifacts/
	@rm Cargo.lock
	@echo "Done!"

###############################################################
## PROJECT UTILITIES                                         ##
###############################################################
.PHONY:
update-deps:
	@rm Package.resolved
	xcodebuild -resolvePackageDependencies

.PHONY:
format:
	${SWIFT} package plugin --allow-writing-to-package-directory swiftformat .
