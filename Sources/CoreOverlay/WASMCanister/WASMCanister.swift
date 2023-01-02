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

    init(module: URL) throws {
        
        do {
            wasmText = try String(contentsOf: module, encoding: .utf8)
        }
        catch {
            COLogger.info("Failed to load WASM text from file!")
            throw NSError()
        }
    }

    // Executer
    public func execute(_ mainFx: String) throws {
        let cWasmText: UnsafeMutablePointer<CChar>? = try? toCStr(from: wasmText)
        let cMainFx: UnsafeMutablePointer<CChar>? = try? toCStr(from: mainFx)
        
        c_exec_wasm_text_module(cWasmText, cMainFx)
    }
}
