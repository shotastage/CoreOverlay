//
//  WebAssembly.swift
//
//
//  Created by Shota Shimazu on 2022/07/20.
//

import Foundation
import WasmInterpreter

/// WASM runtime backend selection
public enum WARuntimeBackend {
    /// WASM3 is fast WebAssembly text interpreter
    case wasm3

    /// Default WASM backend
    case wasmer

    /// JavaScriptCore is JavaScript VM for Apple-platforms especially.
    case jscore
}

/// WASM Canister arguments wrapped object.
public struct WARunnerArguments {
    let variables: [Data]
}

/// WASM canister returns tuple object.
public struct WARunnerReturns {
    let store: [Int: Data]
}

/// WASMRunner protocol
public protocol WASMRunner {
    var programLoad: [UInt8] { get set }
    func run(args: WARunnerArguments) -> WARunnerReturns
}

/// WASM runtime exposed object.
open class WASMCanister {
    let backend: WARuntimeBackend = .wasm3

    func initializeRunner(url: URL) -> some WASMRunner {
        switch backend {
        case .wasm3:
            return try! WASM3Runner(file: url)
        case .wasmer:
            fatalError("Wasmer is currently unsupported!")
        case .jscore:
            fatalError("JavaScriptCore is currently unsupported!")
        }
    }
}
