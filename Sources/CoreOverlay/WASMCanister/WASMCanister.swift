//
//  WASMCanister.swift
//
//
//  Created by Shota Shimazu on 2022/11/29.
//

import Foundation
import WasmInterpreter
import OverlayEngine
import OverlayFundation

public struct WASMModule {
    // WASM VM interpreter instance
    private let _vm: WasmInterpreter

    // bit loaded
    private let programLoad: [UInt8]

    // Initializers
    public init(file _: URL) throws {
        programLoad = [UInt8](Data(base64Encoded: "base64")!)
        _vm = try WasmInterpreter(module: programLoad)
    }

    // Executer
    @discardableResult
    public func execute(_ first: Int, _ second: Int) throws -> Int {
        Int(try _vm.call("main", Int32(first), Int32(second)) as Int32)
    }

    func testable(wasmText: String, mainFx: String) {
        let cWasmText: UnsafeMutablePointer<CChar> = makeCString(from: wasmText)
        let cMainFx: UnsafeMutablePointer<CChar> = makeCString(from: mainFx)
        
        c_exec_wasm_text_module(cWasmText, cMainFx)
    }
}
