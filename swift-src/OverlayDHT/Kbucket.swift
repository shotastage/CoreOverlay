//
//  KBucket.swift
//  
//
//  Created by Shota Shimazu on 2023/01/08.
//

import Foundation


public struct KBucket {
    var nodes: [KNode]
    var capacity: Int

    public init(capacity: Int) {
        self.capacity = capacity
        self.nodes = []
    }

    /// Add function is main procedures to add new peer to get involved a new DHT network
    mutating func add(id: Int, address: String) {
        self.nodes.append(KNode(id: id, address: (address, 913)))
    }

    /// Remove peer from KBucket
    mutating func remove() {
        print("Now under construction...")
        self.nodes.remove(at: 0)
    }
}
