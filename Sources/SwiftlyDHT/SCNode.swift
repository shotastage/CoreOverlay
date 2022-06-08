//
//  SCNode.swift
//  
//
//  Created by Shota Shimazu on 2022/06/07.
//

import Foundation
import Dispatch


public protocol KademliaNode {
    func ping()
    func findNode()
    func store()
    func findValue()
}

open class SCCNode: KademliaNode {
    public init() {
    }

    public func ping() {
        KDHTLogger.info("PING")
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
