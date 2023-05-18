//
//  Store.swift
//  
//
//  Created by Shota Shimazu on 2023/01/13.
//

import Foundation


class KVStore {
    var ht: [String: Data]
    var isOrigin: [String: Bool]

    init() {
        self.ht = [String: Data]()
        self.isOrigin = [String: Bool]()
    }

    func get(key: String) -> (Data?, Bool) {
        if let val = self.ht[key] {
            return (val, true)
        }
        return (nil, false)
    }

    func add(key: String, val: Data, isOrigin: Bool) {
        self.ht[key] = val
        self.isOrigin[key] = isOrigin
    }

    struct KV {
        var key: String
        var val: Data
        var isOrigin: Bool
    }

    func iterator() -> AnyIterator<KV> {
        var keys = Array(self.ht.keys)
        keys.shuffle()
        var index = 0
        return AnyIterator {
            guard index < keys.count else { return nil }
            let key = keys[index]
            index += 1
            return KV(key: key, val: self.ht[key]!, isOrigin: self.isOrigin[key]!)
        }
    }
}
