//
//  KBucket.swift
//  
//
//  Created by Shota Shimazu on 2023/01/08.
//

import Foundation


public struct KPeer {
    let kid: String
    let peer: KPeer
}

public struct KBucket {
    let k: Int = 20
    let peers: [KPeer] = []
    
    public init() {
        k = 20 // This is initial value without initialization.
    }

    
    /// Add function is main procedures to add new peer to get involved a new DHT network
    func add() {
        
    }

    func remove() {
        
    }

    func count() -> Int {
        peers.count
    }
}
