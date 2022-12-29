//
//  WasmNortlize.swift
//
//
//  Created by Shota Shimazu on 2022/12/20.
//

import Foundation
import OverlayFundation

struct NotarlizeTable {
    let objectIdentifier: String
}

class WasmNotarized {
    func nortalize() {
        COLogger.info("Initializing wasm package notarization...")
    }

    func accept() {
        COLogger.info("Accept notarizing...")
    }

    func deny() {
        COLogger.info("Deny notarizing...")
    }
}
