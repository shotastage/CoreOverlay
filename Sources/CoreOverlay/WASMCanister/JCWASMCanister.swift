//
//  WASMCanister.swift
//  
//
//  Created by Shota Shimazu on 2022/06/21.
//

import Foundation
import JavaScriptCore


open class WASMJSContext {

    public func loadWasmModule(data: Data) {
        //
    }
}


/// JCWASMCanister is container of WebAssembly module
open class JCWASMCanister {
    
    /// Wasm support check JavaScript API
    let testWasm = """
        if (typeof WebAssembly !== 'undefined') {
            console.log("Hello, Wasm.");
        } else {
            console.log("No Wasm for you!");
        }
    """
    
    /// Load simple WASM program
    let loadScript = """
     var importObject = { wasi_snapshot_preview1: { proc_exit: arg => console.log(arg) } };
        WebAssembly.instantiateStreaming(
          fetch('hello.wasm'), importObject
        ).then(obj => document.getElementById("output").value = obj.instance.exports.main());
    """

    let jsContext = JSContext()

    var loadedModules: Array<String> = ["sample.wasm"]

    init() {
        fatalError("This object is currently unsupported.")
    }

    public func load(module: String) {
        loadedModules.append(module)
    }

    func exec() {
        jsContext?.evaluateScript(testWasm)
    }
}
