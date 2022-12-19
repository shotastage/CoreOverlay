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

| Package Name | Status |  Description  |
|:-------------|:------:|---------------|
| CoreOverlay  | 🔴     | Main package of this framework |
| CLI          | 🟡     | Commandline tools for CoreOverlay package |
| Runtime      | 🔴     | WebAssembly embeded runtime |
| COLibs       | 🟡     | CoreOverlay utility package    |
| CLevelDB     | 🟡     | LevelDB bridge    |
| WASM3 Backend| 🟡     | WASM3 bridging layer for CoreOverlay     |
| CWasmer      | 🔴     | Wasmer bridging layer for CoreOverlay    |

## License

This software is licensed under the MIT, see [LICENSE](LICENSE) for detail.
