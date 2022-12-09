//
//  WebAssembly.swift
//
//
//  Created by Shota Shimazu on 2022/07/20.
//

import Foundation
import WasmInterpreter


public enum WARuntimeBackend {
    case wasmer
    case wasm3
    case jscore
    case llvm
}

open class WASMCanister {
    let backend: WARuntimeBackend = .wasm3

    // WASM VM interpreter instance
    private let _vm: WasmInterpreter

    // bit loaded
    private let programLoad: [UInt8]

    // Initializers
    public init(file: URL) throws {
        programLoad = [UInt8](Data(base64Encoded: "base64")!)
        _vm = try WasmInterpreter(module: programLoad)
    }

    // Executer
    public func execute(_ first: Int, _ second: Int) throws -> Int {
        Int(try _vm.call("main", Int32(first), Int32(second)) as Int32)
    }
}


class JSCoreRuntime: WASMCanister {
    
}
