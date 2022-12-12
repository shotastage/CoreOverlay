//
//  Kademlia.swift
//
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Foundation
import OverlayFundation
import Protobuf_Generated

public actor KademliaNode {
    var nodeId: Int = 100
    let k: Int = 1
    let kbuckets: [[SocketClient]] = [[SocketClient(ip: "192.16.8.1", port: 8080)]]

    /// Initializer with auto IP designation
    /// - Parameter nodeId: Designated kademlia node id
    init(nodeId: Int?) {
        guard let id = nodeId else {
            return
        }

        self.nodeId = id

        fatalError("Now under construction")
    }

    /// Initializer with
    /// - Parameters:
    ///   - nodeId: Designated kademlia node id
    ///   - ip: Designated IP address
    init(nodeId: Int?, ip _: String) {
        guard let id = nodeId else {
            return
        }

        self.nodeId = id

        fatalError("Now under construction")
    }

    /// Ping is Kademlia protocol method to check active status for designated node
    /// - Parameter nodeId: Integer node id
    public func ping(nodeId _: Int) {
        COLogger.info("PING")
        fatalError("Not implemented")
    }

    /// FindNode is one of Kademlia protocol method
    public func findNode() {
        COLogger.info("FIND_NODE")
        fatalError("Not implemented")
    }

    /// Store is one of Kademlida protocol method to write record on DHT.
    /// - Parameters:
    ///   - key: String key identifier
    ///   - value: Any data conforming to Swift Data type.
    public func store(key _: String, value _: Data) {
        COLogger.info("STORE")
        fatalError("Not implemented")
    }

    /// findValue is one of Kademlia protocol method to find stored value with the key as identifier
    /// - Parameter forKey: Search key identifier
    public func findValue(forKey: String) {
        COLogger.info("FIND_VALUE with Key: \(forKey)")
        fatalError("Not implemented")
    }
}
