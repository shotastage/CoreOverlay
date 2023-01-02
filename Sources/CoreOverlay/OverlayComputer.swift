//
//  OverlayComputer.swift
//
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Foundation
import OverlayFundation
import Runtime

open class OverlayComputer {
    /// storedObject is gathered instance
    var storedObject = OverlayComputerSharedInstances(runnder: [:], receivers: [:], servers: [:])
    var package: URL?

    public init() {
        storedObject.receivers.updateValue(KademliaNode(nodeId: 1234), forKey: "genesis")
        storedObject.servers.updateValue(KademliaNode(nodeId: 1235), forKey: "genesis")
        package = nil
    }

    public init(location: URL) {
        storedObject.receivers.updateValue(KademliaNode(nodeId: 1234), forKey: "genesis")
        storedObject.servers.updateValue(KademliaNode(nodeId: 1235), forKey: "genesis")
        package = location
    }

    public func configure() {
        COLogger.info("Not implemented")
    }

    public func start() {
        COLogger.info("Starting CoreOverlay internal server...")

        let wat = """
(module
(type $t0 (func (param i32) (result i32)))
(func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
get_local $p0
i32.const 1
i32.add))
"""
        
        do {
            let wasm = try WASMModule(module: wat)
            try! wasm.execute("main")
        } catch {
            print("Failed to register WASM artifcats.")
        }
    }

    public func shutdown() {
        COLogger.info("Shutting down server...")
    }

    public func registerArtifacts(url: URL) {
        package = url
    }
}

public struct OverlayComputerSharedInstances {
    var runnder: [String: WASMCanister]
    var receivers: [String: KademliaNode]
    var servers: [String: KademliaNode]
}
