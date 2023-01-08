//
//  KBucket.swift
//  
//
//  Created by Shota Shimazu on 2023/01/08.
//

import Foundation


public struct KPeer {
    let kid: String
    // let peer: KPeer
}

public struct KBucket {

    /// Initial K variables number
    var k: Int = 20

    
    /// Peers store
    let peers: [KPeer] = []

    
    /// Peers number counter
    /// Usage:
    /// kBucketInstance().count
    public var count: Int {
        peers.count
    }

    public init() {
        k = 20 // This is initial value without initialization.
    }
    
    /// Add function is main procedures to add new peer to get involved a new DHT network
    func add() {
        print("Now under construction...")
    }

    /// Remove peer from KBucket
    func remove() {
        print("Now under construction...")
    }
}
