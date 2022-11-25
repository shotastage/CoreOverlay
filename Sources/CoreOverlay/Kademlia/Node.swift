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
