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

// Bootstrap type is representation for node accesor especially for knowing node.
public struct Bootstrap {
    let addr: String
    let port: UInt16
    
    public init(addr: String, port: UInt16) {
        self.addr = addr
        self.port = port
    }
}

public actor OverlayDHT {
    let connection = UDPConnection(on: 1234)
    let subscription: AnyCancellable

    let node: KNode
    var k: Int = 20
    let kbuckets: [KNode]

    public init(k: Int = 20, bootstrap: Bootstrap) {
        node = KNode(id: OverlayDHTUtils.randomID(), address: (bootstrap.addr, bootstrap.port))
        self.k = k
        kbuckets = []
        subscription = connection.objectWillChange
            .sink {
                print("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
                print("Object has changed")
                print("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
        }
    }

    deinit {
        connection.cancel()
    }

    func bootstrap() {
        // connection.connecnt(host: "", port: 1234)
    }

    /// Basic Kademlia Methods
    ///
    func ping(id: Int) async throws {
        for bucket in kbuckets {
            if bucket.id == id {
                connection.createConnection(host: bucket.address.host, port: 19000)
            }
        }
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
