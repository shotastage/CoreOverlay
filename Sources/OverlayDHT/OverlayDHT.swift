//
//  OverlayDHT.swift
//  
//
//  Created by Shota Shimazu on 2023/01/07.
//

import Foundation
import Combine


enum KademliaRPCProtocol: Int {
    case ping = 0
    case findNode = 1
    case findValue = 2
    case store = 3
}


public actor OverlayDHT {

    @Published var observingConnection = ""
    
    let node: KNode
    var k: Int = 20
    let kbuckets: [KNode]
    let connection = SocketClient()

    init(k: Int = 20) {
        node = KNode(id: OverlayDHTUtils.randomID(), address: ("localhost", 8080))
        self.k = k
        kbuckets = []
    }

    deinit {
        connection.disconnect()
    }

    func bootstrap() {
        connection.connecnt(host: "", port: 1234)
    }

    /// Basic Kademlia Methods
    ///
    func ping(id: Int) async {
        
        fatalError("Not implemented")
    }

    func findNode() async {
        fatalError("Not implemented")
    }

    func store() async {
        fatalError("Not implemented")
    }

    func findValue() async {
        fatalError("Not implemented")
    }

    /// Extra implemented utilities
    ///
    func nodeLookup() {
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
