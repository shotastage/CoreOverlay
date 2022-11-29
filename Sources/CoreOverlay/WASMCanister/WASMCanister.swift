//
//  WASMCanister.swift
//  
//
//  Created by Shota Shimazu on 2022/11/29.
//

import WasmInterpreter
import Foundation


struct WASMContainer {
    private let _vm: WasmInterpreter

    init() throws {
        _vm = try WasmInterpreter(module: WASMContainer.wasm)
    }

    func add(_ first: Int, _ second: Int) throws -> Int {
        Int(try _vm.call("add", Int32(first), Int32(second)) as Int32)
    }

    private static var wasm: [UInt8] {
        
        
        /*
         guard let fileURL = Bundle.main.url(forResource: "program", withExtension: "wb64")  else {
             //fatalError("File Not Found")
             
             
         }
         guard let fileContents = try? String(contentsOf: fileURL) else {
             print("FIle content reading error")
             //fatalError("File Parsing Error")
         }
         
         */
        
        let base64 = "AGFzbQEAAAABBwFgAn9/AX8DAgEABwcBA2FkZAAACgkBBwAgACABags="
        return [UInt8](Data(base64Encoded: base64)!)
    }
}
