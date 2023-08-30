//
//  KNode.swift
//
//
//  Created by Shota Shimazu on 2023/08/30.
//
import Foundation

// 160-bit node ID represented as a 20-byte array
typealias NodeID = [UInt8]

// Calculate XOR distance between two NodeIDs
func xorDistance(_ a: NodeID, _ b: NodeID) -> NodeID {
    return zip(a, b).map { $0.0 ^ $0.1 }
}

// Convert NodeID to UInt160 for easier comparison
func toUInt160(_ id: NodeID) -> UInt {
    // Assuming NodeID is 4 bytes for simplicity
    return UInt(bigEndian: Data(id).withUnsafeBytes { $0.load(as: UInt.self) })
}

// Kademlia Node
struct Node {
    let id: NodeID
    var buckets: [[Node]] // Each bucket contains nodes at a certain distance range
    
    init(id: NodeID) {
        self.id = id
        self.buckets = Array(repeating: [], count: 160)
    }
    
    // Find k closest nodes to a given target ID
    func findClosestNodes(target: NodeID, k: Int) -> [Node] {
        var closestNodes: [Node] = []
        
        for bucket in buckets {
            for node in bucket {
                if closestNodes.count < k {
                    closestNodes.append(node)
                    closestNodes.sort { toUInt160(xorDistance($0.id, target)) < toUInt160(xorDistance($1.id, target)) }
                } else {
                    let farthestCloseNode = closestNodes.last!
                    if toUInt160(xorDistance(node.id, target)) < toUInt160(xorDistance(farthestCloseNode.id, target)) {
                        closestNodes.removeLast()
                        closestNodes.append(node)
                        closestNodes.sort { toUInt160(xorDistance($0.id, target)) < toUInt160(xorDistance($1.id, target)) }
                    }
                }
            }
        }
        
        return Array(closestNodes.prefix(k))
    }
    
    // Insert a node into the appropriate bucket
    mutating func insert(_ node: Node) {
        let distance = xorDistance(self.id, node.id)
        let index = 159 - distance.prefix { $0 == 0 }.count // Find the first differing bit
        buckets[index].append(node)
    }
}
