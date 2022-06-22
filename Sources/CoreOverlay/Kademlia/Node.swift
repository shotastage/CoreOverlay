//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/06/22.
//

import Foundation


public enum NetworkType {
    case `super`
    case communicative
}

open class OverlayNode: KademliaNode {

    var networkType: NetworkType = .super

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
