//
//  OverlayComputer.swift
//
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Foundation
import COLibs


open class OverlayComputer {

    // Reser
    let receiver: KademliaNode?
    let server: KademliaNode?
    var package: URL?

    public init() {
        receiver = KademliaNode(nodeId: 1234)
        server = KademliaNode(nodeId: 43242)
        package = nil
    }

    public init(location: URL) {
        receiver = KademliaNode(nodeId: 1234)
        server = KademliaNode(nodeId: 43242)
        package = location
    }

    public func start() {
        COLogger.info("Starting CoreOverlay internal server...")

        do {
            let wasm = try WASMModule(file: package!)
        }
        catch {
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
