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
    let wasmFile: Data

    init(module: URL) throws {
        
        arch()
        do {
            wasmText = try String(contentsOf: module, encoding: .utf8)
            wasmFile = try Data(contentsOf: module)
        }
        catch {
            COLogger.info("Failed to load WASM text from file!")
            throw NSError()
        }
    }

    // Executer
    public func nativeExecute(_ mainFx: String) throws {
        let cWasmText: UnsafeMutablePointer<CChar>? = try? toCStr(from: wasmText)
        let cMainFx: UnsafeMutablePointer<CChar>? = try? toCStr(from: mainFx)
        
        c_exec_wasm_text_module(cWasmText, cMainFx)
        //- c_exec_wasm_native_module(12, 32)
    }
    
    public func execute(_ mainFx: String) throws {
        let cWasmText: UnsafeMutablePointer<CChar>? = try? toCStr(from: wasmText)
        let cMainFx: UnsafeMutablePointer<CChar>? = try? toCStr(from: mainFx)
        
        c_exec_wasm_text_module(cWasmText, cMainFx)
    }
}
