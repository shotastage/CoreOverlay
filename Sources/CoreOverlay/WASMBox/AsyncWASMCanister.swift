//
//  AsyncWASMBox.swift
//  
//
//  Created by Shota Shimazu on 2023/03/20.
//

import Foundation

actor WasmBoxActor {
    private var internalValue: Int

    init(value: Int) {
        self.internalValue = value
    }
}

extension WasmBoxActor {
    func asyncOperation() async -> Int {
        // Perform some asynchronous operation
        await Task.sleep(UInt64(1_000_000_000)) // Sleep for 1 second
        internalValue += 1
        return internalValue
    }
}
