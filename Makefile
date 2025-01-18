SHELL=/usr/bin/env bash

# Base configuration
ROOT_DIR := $(shell pwd)
SWIFT_DIR := $(ROOT_DIR)/src-swift
ARTIFACTS_DIR := $(ROOT_DIR)/artifacts
VERSION ?= $(shell git describe --tags --always --dirty)
BUNDLE_NAME := bundle-$(VERSION).zip

# Build configuration flags
ENABLE_MAC_CATALYST := 0
ENABLE_IOS_SUPPORT := 0
ENABLE_ANDROID_SUPPORT := 0
ENABLE_WINDOWS_SUPPORT := 0

# Command Definitions
SWIFT := swift
CARGO ?= cargo
GO ?= go
SDKROOT := $(shell xcrun --sdk macosx --show-sdk-path)

# Platform targets
RUST_TARGETS := x86_64-apple-darwin aarch64-apple-darwin
ifdef ENABLE_IOS_SUPPORT
    RUST_TARGETS += aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
endif
ifdef ENABLE_MAC_CATALYST
    RUST_TARGETS += x86_64-apple-ios-macabi aarch64-apple-ios-macabi
endif

# Helper functions
define cleanup_artifacts
	@echo "Cleaning up artifacts..."
	@rm -f $(ARTIFACTS_DIR)/libcoreoverlayengine_macos.a
	@rm -f $(ARTIFACTS_DIR)/libcoreoverlayengine_iossimulator.a
endef

###############################################################
## BUILD ENV SETUP FOR DEVELOPMENT                           ##
###############################################################
.PHONY: setup
setup: check-prerequisites
	$(CARGO) install --force cbindgen
	$(foreach target,$(RUST_TARGETS),rustup target add $(target);)
ifeq ($(ENABLE_MAC_CATALYST), 1)
	$(CARGO) +nightly build --release -Z build-std --target x86_64-apple-ios-macabi
	$(CARGO) +nightly build --release -Z build-std --target aarch64-apple-ios-high-macabi
endif
ifeq ($(ENABLE_ANDROID_SUPPORT), 1)
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add i686-linux-android
	rustup target add x86_64-linux-android
endif

###############################################################
## BUILD ENV SETUP FOR CI                                    ##
###############################################################
.PHONY: setup-ci
setup-ci: check-prerequisites
	$(foreach target,$(RUST_TARGETS),rustup target add $(target);)
ifeq ($(ENABLE_MAC_CATALYST), 1)
	$(CARGO) +nightly build --release -Z build-std --target x86_64-apple-ios-macabi
	$(CARGO) +nightly build --release -Z build-std --target aarch64-apple-ios-high-macabi
endif

###############################################################
## BUILD PROCEDURES                                          ##
###############################################################

.PHONY: check-prerequisites
check-prerequisites:
	@command -v $(SWIFT) >/dev/null 2>&1 || { echo "Swift is not installed"; exit 1; }
	@command -v $(CARGO) >/dev/null 2>&1 || { echo "Cargo is not installed"; exit 1; }
	@command -v $(GO) >/dev/null 2>&1 || { echo "Go is not installed"; exit 1; }

.PHONY: pre-build-preparation
pre-build-preparation:
	@echo "Preparing build..."
	@mkdir -p $(ARTIFACTS_DIR)
	@rm -rf $(ARTIFACTS_DIR)/*

.PHONY: build-main-components
build-main-components:
	$(CARGO) build --release

.PHONY: build-apple-platform-framework
build-apple-platform-framework:
	$(foreach target,$(RUST_TARGETS),$(CARGO) build --release --target $(target);)

.NOTPARALLEL: build-universal-bin
.PHONY: build-universal-bin
build-universal-bin:
	mkdir -p $(ARTIFACTS_DIR)
	lipo -create \
		target/x86_64-apple-darwin/release/libcoreoverlayengine.a \
		target/aarch64-apple-darwin/release/libcoreoverlayengine.a \
		-output $(ARTIFACTS_DIR)/libcoreoverlayengine_macos.a

	lipo -create \
		target/x86_64-apple-ios/release/libcoreoverlayengine.a \
		target/aarch64-apple-ios-sim/release/libcoreoverlayengine.a \
		-output $(ARTIFACTS_DIR)/libcoreoverlayengine_iossimulator.a
ifeq ($(ENABLE_MAC_CATALYST), 1)
	lipo -create \
		target/x86_64-apple-ios-macabi/release/libcoreoverlayengine.a \
		target/aarch64-apple-ios-macabi/release/libcoreoverlayengine.a \
		-output $(ARTIFACTS_DIR)/libcoreoverlayengine_maccatalyst.a
endif

.NOTPARALLEL: build-rust-xcframework
.PHONY: build-rust-xcframework
build-rust-xcframework:
	xcodebuild -create-xcframework \
		-library $(ARTIFACTS_DIR)/libcoreoverlayengine_macos.a -headers ./include/ \
		-library $(ARTIFACTS_DIR)/libcoreoverlayengine_iossimulator.a -headers ./include/ \
		-library ./target/aarch64-apple-ios/release/libcoreoverlayengine.a -headers ./include/ \
		-output $(ARTIFACTS_DIR)/CoreOverlayEngine.xcframework

ifeq ($(ENABLE_MAC_CATALYST), 1)
	xcodebuild -create-xcframework \
		-library $(ARTIFACTS_DIR)/libcoreoverlayengine_macos.a -headers ./include/ \
		-library $(ARTIFACTS_DIR)/libcoreoverlayengine_iossimulator.a -headers ./include/ \
		-library $(ARTIFACTS_DIR)/libcoreoverlayengine_maccatalyst.a -headers ./include/ \
		-library ./target/aarch64-apple-ios/release/libcoreoverlayengine.a -headers ./include/ \
		-output $(ARTIFACTS_DIR)/CoreOverlayEngine.xcframework
endif

.PHONY: build-swift
build-swift:
	cd $(SWIFT_DIR) && ${SWIFT} build

.PHONY: build-artifacts
build-artifacts:
	@echo "Zipping artifacts..."
	@zip -r $(ARTIFACTS_DIR)/$(BUNDLE_NAME) $(ARTIFACTS_DIR)/CoreOverlayEngine.xcframework

.PHONY: build-finalize
build-finalize:
	@echo "Cleaning up..."
	$(call cleanup_artifacts)
	@echo "Build complete!"
	@echo "Information:"
	@echo "Artifacts are located in $(ARTIFACTS_DIR)"
	@openssl dgst -sha256 $(ARTIFACTS_DIR)/$(BUNDLE_NAME)

###############################################################
## GATHERED ALL BUILD PROCEDURE                              ##
###############################################################
.PRECIOUS: $(ARTIFACTS_DIR)/%.a
.PHONY: build
build: check-prerequisites pre-build-preparation build-main-components build-apple-platform-framework build-universal-bin build-rust-xcframework build-artifacts build-swift build-finalize

###############################################################
## GATHERED ALL BUILD PROCEDURE FOR RELEASE                  ##
###############################################################
.PHONY: release
release:
	cd $(SWIFT_DIR) && ${SWIFT} build -c release

###############################################################
## GATHERED ALL BUILD PROCEDURE FOR CI                       ##
###############################################################
.PHONY: build-ci
build-ci: check-prerequisites pre-build-preparation build-main-components build-apple-platform-framework build-universal-bin build-rust-xcframework build-artifacts build-swift build-finalize
	rm -rf $(ARTIFACTS_DIR)/CoreOverlayEngine.xcframework

###############################################################
## CLEAN PROJECT WORKSPACE                                   ##
###############################################################
.PHONY: clean
clean:
	@echo "Cleaning project..."
	@rm -rf $(SWIFT_DIR)/.build/
	@rm -rf $(SWIFT_DIR)/.swiftpm/xcode/
	@cargo clean
	@rm -rf $(ARTIFACTS_DIR)
	@rm -f Cargo.lock
	@echo "Done!"

###############################################################
## PROJECT UTILITIES                                         ##
###############################################################
.PHONY: update-deps
update-deps:
	@rm -f $(SWIFT_DIR)/Package.resolved
	cd $(SWIFT_DIR) && xcodebuild -resolvePackageDependencies

.PHONY: format
format:
	cd $(SWIFT_DIR) && ${SWIFT} package plugin --allow-writing-to-package-directory swiftformat .

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  setup          - Set up development environment"
	@echo "  setup-ci       - Set up CI environment"
	@echo "  build         - Build all components"
	@echo "  build-ci      - Build all components in CI environment"
	@echo "  clean         - Clean workspace"
	@echo "  release       - Build release version"
	@echo "  format        - Format Swift code"
	@echo "  update-deps   - Update dependencies"
	@echo "  help          - Show this help message"

.PHONY: print-debug
print-debug:
	@echo "RUST_TARGETS: $(RUST_TARGETS)"
	@echo "ARTIFACTS_DIR: $(ARTIFACTS_DIR)"
	@echo "VERSION: $(VERSION)"
	@echo "BUNDLE_NAME: $(BUNDLE_NAME)"
