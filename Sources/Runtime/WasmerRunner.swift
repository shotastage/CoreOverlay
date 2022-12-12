//
//  WasmerRunner.swift
//
//
//  Created by Shota Shimazu on 2022/10/25.
//

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

    init() {
        fatalError("Now under construction...")
    }

    public func run(args _: WARunnerArguments) -> WARunnerReturns {
        return WARunnerReturns(store: [0: "Sample"])
    }
}
