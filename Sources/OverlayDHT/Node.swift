//
//  Node.swift
//
//
//  Created by Shota Shimazu on 2023/01/09.
//

import Foundation


    static func genDefaultPort() -> String {
        var port = "1234"
        if let envPort = ProcessInfo.processInfo.environment["PORT"] {
            port = envPort
        }
        return port
    }


public struct NodeOption {
    let id: Int
    let ip: String
    let port: String
    let expirationTime: Date
    let refreshTime: Date

    init(ip: String, port: String, exprTime: Date, refTime: Date) throws {
        guard let ip = ip, let port = port else {
            throw NodeError.invalidParameters
        }
        self.id = OverlayDHTUtils.randomID()
        self.ip = ip
        self.port = port
        self.expirationTime = exprTime
        self.refreshTime = refTime
    }
}


/// DHTNode()
///
///

public struct KNode {
    let id: Int
    let address: (host: String, port: UInt16)

    init?(id: Int, address: (String, UInt16)) {
        guard id >= 0 else {
            return nil
        }
        self.id = id
        self.address = address
    }

    func distance(to other: KNode) -> Int {
        return OverlayDHTUtils.xor(self.id, other.id)
    }
}

public class KademliaNode {
    var id: Int // unique id for the node
    var option: NodeOption // kademlia node option
    var routingTable: [KademliaNode] // array of other nodes in the network
    var kbucket: [KademliaNode] // array of other nodes in the network

    init(option opt: NodeOption) {
        self.id = OverlayDHTUtils.randomID()
        self.routingTable = []
        self.option = opt
        self.kbucket = []
    }

    // Function to calculate the XOR distance between two nodes
    func distance(otherNode: KademliaNode) -> Int {
        return OverlayDHTUtils.xor(self.id, otherNode.id)
        // return self.id ^ otherNode.id
    }

    // Function to add a node to the routing table
    func addNode(node: KademliaNode) {
        routingTable.append(node)
    }

    // Function to perform a lookup operation on the network
    func lookup(key: Int) -> Int? {
        // Find the closest nodes to the key
        let closestNodes = routingTable.sorted { (node1, node2) -> Bool in
            return self.distance(otherNode: node1) < self.distance(otherNode: node2)
        }

        // Query the closest nodes for the value associated with the key
        for node in closestNodes {
            if let value = node.getValue(forKey: key) {
                return value
            }
        }

        // If the key is not found, return nil
        return nil
    }


    // Function to perform a publish operation on the network
    func publish(key: Int, value: Int) {
        // This is just a placeholder - in a real implementation,
        // the publish operation would involve advertising the key-value
        // pair to other nodes in the network
    }

    // Function to update information about other nodes in the network
    func update() {
        // This is just a placeholder - in a real implementation,
        // the update operation would involve exchanging information
        // with other nodes in the network to refresh the routing table
    }

    func getValue(forKey: Int) -> Int? {
        return nil
    }
}
