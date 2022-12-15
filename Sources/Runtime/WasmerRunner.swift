//
//  WasmerRunner.swift
//
//
//  Created by Shota Shimazu on 2022/10/25.
//

import CWasmer
import Foundation

/*
 import Wasmer

 public enum Wasmer {}

 public extension Wasmer {

   static var version : ( major: Int, minor: Int, patch : Int ) {
     return ( Int(wasmer_version_major()),
              Int(wasmer_version_minor()),
              Int(wasmer_version_patch()) )
   }
 }
 */

open class WasmerRunner: WASMRunner {
    public var programLoad: [UInt8] = []


    //let engine: OpaquePointer! = wasm_engine_new()
    //let store: OpaquePointer!

    init() {
        //store = wasm_store_new(self.engine)
    }

    public func run(args _: WARunnerArguments) -> WARunnerReturns {
        WARunnerReturns(store: [0: Data("Sample".utf8)])
    }
}
