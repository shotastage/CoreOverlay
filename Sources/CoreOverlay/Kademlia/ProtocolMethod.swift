//
//  ProtocolMethod.swift
//  
//
//  Created by Shota Shimazu on 2022/06/22.
//

import Foundation


public protocol KademliaNode {
    func ping(nodeId: Int)
    func findNode()
    func store()
    func findValue()
}


extension KademliaNode {
    
    func ping(nodeId: Int) {
        COLogger.info("Ping to node \(nodeId)")
    }
}
