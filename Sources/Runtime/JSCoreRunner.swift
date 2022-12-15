//
//  JSCoreRunner.swift
//
//
//  Created by Shota Shimazu on 2022/12/12.
//

import Foundation

open class JSCoreRunner: WASMRunner {
    public var programLoad: [UInt8] = []

    init() {
        fatalError("Now under construction...")
    }

    public func run(args _: WARunnerArguments) -> WARunnerReturns {
        WARunnerReturns(store: [0: Data("Sample".utf8)])
    }
}
