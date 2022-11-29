//
//  WASMCanister.swift
//  
//
//  Created by Shota Shimazu on 2022/06/21.
//

import JavaScriptCore
import WasmInterpreter


open class JCWASMCanister {
    let testWasm = """
        if (typeof WebAssembly !== 'undefined') {
            console.log("Hello, Wasm.");
        } else {
            console.log("No Wasm for you!");
        }
    """
    
    
    let loadScript = """
     var importObject = { wasi_snapshot_preview1: { proc_exit: arg => console.log(arg) } };
        WebAssembly.instantiateStreaming(
          fetch('hello.wasm'), importObject
        ).then(obj => document.getElementById("output").value = obj.instance.exports.main());
    """

    let jsContext = JSContext()

    let carriedBin = "sample.wasm"

    init() {
        //
    }

    func exec() {
        jsContext?.evaluateScript(testWasm)
    }
}
