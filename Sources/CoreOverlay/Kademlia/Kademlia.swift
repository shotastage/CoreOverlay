//
//  Kademlia.swift
//  
//
//  Created by Shota Shimazu on 2022/06/08.
//

import COLibs


public actor Kademlia {
    let nodeId: String = ""
    let k: Int = 1
    let kbuckets: [[SocketClient]] = [[ SocketClient(ip: "192.16.8.1", port: 8080) ]]


    public func ping() {
        COLogger.info("ping")
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
