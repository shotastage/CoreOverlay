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
    case wasmer
    case wasm3
    case jscore
}


/// WASM Canister arguments wrapped object.
public struct WARunnerArguments {
    let variables: Dictionary<String, String>
}


/// WASM canister returns tuple object.
public struct WARunnerReturns {
    let store: Dictionary<Int, String>
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
            return try! WASM3Runner(file: url)
        case .jscore:
            return try! WASM3Runner(file: url)
        }
    }
}

/// WASM runners
open class WASM3Runner: WASMRunner {

    // WASM VM interpreter instance
    private let _vm: WasmInterpreter

    public var programLoad: [UInt8]


    // Initializers
    public init(file: URL) throws {
        programLoad = [UInt8](Data(base64Encoded: "base64")!)
        _vm = try WasmInterpreter(module: programLoad)
    }

    // Executer
    public func execute(_ first: Int, _ second: Int) throws -> Int {
        Int(try _vm.call("main", Int32(first), Int32(second)) as Int32)
    }


    public func run(args: WARunnerArguments) -> WARunnerReturns {
        // Now under construction...
        return WARunnerReturns(store: [0: "SampleStore"])
    }
}


class JSCoreRuntime: WASMRunner {
    var programLoad: [UInt8] = []
    
    init() {
        print("Now under construction...")
    }

    func run(args: WARunnerArguments) -> WARunnerReturns {
        return WARunnerReturns(store: [0: "Sample"])
    }
}
