//
//  WASMCanister.swift
//  
//
//  Created by Shota Shimazu on 2022/11/29.
//

import WasmInterpreter
import Foundation


public struct WASMModule {

    // WASM VM interpreter instance
    private let _vm: WasmInterpreter

    // bit loaded
    private let programLoad: [UInt8]

    // Initializers
    init(file: URL) throws {
        programLoad = [UInt8](Data(base64Encoded: "base64")!)
        _vm = try WasmInterpreter(module: programLoad)
    }

    // Executer
    func execute(_ first: Int, _ second: Int) throws -> Int {
        Int(try _vm.call("main", Int32(first), Int32(second)) as Int32)
    }
}
