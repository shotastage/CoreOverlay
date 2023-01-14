# CoreOverlay

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/shotastage/CoreOverlay/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/shotastage/CoreOverlay/tree/main)

>> This framework is now under construction...

This framework provides utilities that make or integrate structured overlay network.


## Overlay Feature

- Kademlia Based Protocol
- Fast consensus algs
- Abstracted P2P IO


## Package Structure

✅ Work 🔴 Not work 🟡 Partially work

| Package Name     | Status |  Description  |
|:-----------------|:------:|---------------|
| CoreOverlay      | 🟡     | Main package of this framework |
| CLI              | 🟡     | Commandline tools for CoreOverlay package |
| Runtime          | 🟡     | WebAssembly embeded runtime |
| OverlayFundation | 🟡     | CoreOverlay utility package |
| CLevelDB         | 🟡     | LevelDB bridge              |
| CWasmer          | ⚪️     | Standalone wasmer runtime will be deprecated due to JIT-compiler restriction policy.    |


## Self Build Instruction

>> Currently, we supports iOS `*.framework` build or Swift Package distribution.

To make standalone frmework, you can use `Makefile` running `make build`.


## License

This software is licensed under the MIT, see [LICENSE](LICENSE) for detail.
