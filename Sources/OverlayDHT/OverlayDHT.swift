//
//  OverlayDHT.swift
//  
//
//  Created by Shota Shimazu on 2023/01/07.
//

import Foundation


protocol Kademlia {
    func findNode()
    func store()
    func findValue()
}

open class OverlayDHT: Kademlia {

    let node: KNode
    var k: Int = 20
    let kbuckets: [KBucket]

    init(k: Int = 20) {
        node = KNode(id: OverlayDHTUtils.randomID(), address: ("localhost", 8080))
        self.k = k
        kbuckets = []
    }

    deinit {}

    func nodeLookup() {
        fatalError("Not implemented")
    }

    func store() {
        fatalError("Not implemented")
    }

    func findNode() {
        fatalError("Not implemented")
    }

    func ping() {
        fatalError("Not implemented")
    }
    
    func findValue() {
        fatalError("Not implemented")
    }

    func iterativeFindNode() {
        fatalError("Not implemented")
    }
    
    func iterativeStore() {
        fatalError("Not implemented")
    }

    func iterativeFindValue() {
        fatalError("Not implemented")
    }
}
