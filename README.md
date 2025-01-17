# CoreOverlay

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/shotastage/CoreOverlay/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/shotastage/CoreOverlay/tree/main)

A utility framework for building and integrating structured overlay networks.

> **Note**: This framework is currently under development

## Key Features

CoreOverlay provides the following capabilities:

* Kademlia-based protocol implementation
* Fast consensus algorithms
* Abstracted P2P I/O interface

## Package Structure

Development status of each package:

✅ Working | 🔴 Not working | 🟡 Partially working

| Package Name | Status | Description |
|------------|------|------|
| CoreOverlay | 🟡 | Main framework package |
| CLI | 🟡 | Command-line tools for CoreOverlay |
| Runtime | 🟡 | WebAssembly embedded runtime |
| OverlayFundation | 🟡 | CoreOverlay utility package |
| RS-Wasmer | ⚪️ | Standalone Wasmer runtime (to be deprecated due to JIT compiler restriction policy) |

## Build Instructions

Currently supported distribution formats:
- iOS `.framework`
- Swift Package

To create a standalone framework, run the following command:

```bash
make build
```

## License

This software is released under the MIT License. See [LICENSE](LICENSE) for details.
