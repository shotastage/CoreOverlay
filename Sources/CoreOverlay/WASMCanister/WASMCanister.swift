//
//  WASMCanister.swift
//
//
//  Created by Shota Shimazu on 2022/11/29.
//

import Foundation
import CBreeze
import OverlayEngine
import OverlayFundation

public struct WASMModule {

    let wasmText: String

    init(module: String) {
        wasmText = module
    }

    // Executer
    public func execute(_ mainFx: String) {
        let cWasmText: UnsafeMutablePointer<CChar> = toCString(from: wasmText)
        let cMainFx: UnsafeMutablePointer<CChar> = toCString(from: mainFx)
        
        c_exec_wasm_text_module(cWasmText, cMainFx)
    }
}
