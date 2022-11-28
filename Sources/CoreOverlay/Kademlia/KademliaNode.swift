//
//  Kademlia.swift
//  
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Protobuf_Generated
import COLibs


public actor KademliaNode {
    var nodeId: Int = 100
    let k: Int = 1
    let kbuckets: [[SocketClient]] = [[ SocketClient(ip: "192.16.8.1", port: 8080) ]]

    init(nodeId: Int?) {
        guard let id = nodeId else {
            return
        }

        self.nodeId = id
    }

    public func ping(nodeId: Int) {
        COLogger.info("PING")
    }

    public func findNode() {
        COLogger.info("FIND_NODE")
    }

    public func store() {
        COLogger.info("STORE")
    }
    
    public func findValue() {
        COLogger.info("FIND_VALUE")
    }
}
