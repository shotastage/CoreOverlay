//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Foundation


public actor Kademlia {
    let nodeId: String = ""
    let k: Int = 1
    let kbuckets: [[SocketClient]] = [[ SocketClient(ip: "192.16.8.1", port: 8080) ]]


    public func ping() {
        KDHTLogger.info("ping")
    }

    public func findNode() {
        KDHTLogger.info("FIND_NODE")
    }

    public func store() {
        KDHTLogger.info("STORE")
    }
    
    public func findValue() {
        KDHTLogger.info("FIND_VALUE")
    }
}
