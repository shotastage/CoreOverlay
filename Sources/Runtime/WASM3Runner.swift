//
//  File.swift
//
//
//  Created by Shota Shimazu on 2022/12/12.
//

import Foundation
import WasmInterpreter

/// WASM runners
open class WASM3Runner: WASMRunner {
    // WASM VM interpreter instance
    private let _vm: WasmInterpreter

    public var programLoad: [UInt8]

    // Initializers
    public init(file _: URL) throws {
        programLoad = [UInt8](Data(base64Encoded: "base64")!)
        _vm = try WasmInterpreter(module: programLoad)
    }

    // Executer
    public func execute(_ first: Int, _ second: Int) throws -> Int {
        Int(try _vm.call("main", Int32(first), Int32(second)) as Int32)
    }

    public func run(args _: WARunnerArguments) -> WARunnerReturns {
        // Now under construction...
        return WARunnerReturns(store: [0: "SampleStore"])
    }
}
