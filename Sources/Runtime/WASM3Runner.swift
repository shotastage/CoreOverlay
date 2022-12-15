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
    public func execute(_ args: WARunnerArguments) throws -> some WasmTypeProtocol {
        
        guard let contents = try? _vm.call(
            "main",
            Int32(args.variables[0]),
            Int32(args.variables[1]),
            Int32(args.variables[2])
        ) else {
            return 12
        }
        
        return contents
    }

    public func run(args _: WARunnerArguments) -> WARunnerReturns {
        // Now under construction...
        WARunnerReturns(store: [0: Data("Sample".utf8)])
    }
}
