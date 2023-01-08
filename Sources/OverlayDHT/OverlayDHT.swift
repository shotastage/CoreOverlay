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

    let kid: String
    let kbuckets: [KBucket]

    init() {
        kid = OverlayDHTUtils.randomID()
        kbuckets = []
    }
    
    deinit {}

    func bootstrap() {
        
    }

    func findNode() {
        // Now under construction
    }
    
    func store() {
        // Now under construction
    }
    
    func findValue() {
        // Now under construction
    }

    func add() {
        
    }
}
