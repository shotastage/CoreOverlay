//
//  DHT+Utils.swift
//  
//
//  Created by Shota Shimazu on 2023/01/07.
//

import Foundation
import CommonCrypto


class KademliaSpecifications {
    let bitSpace: Int = 160
    let k: Int = 20
}

class OverlayDHTUtils {

    public static func sha1(string: String) -> Int {
        let data = Data(string.utf8)
        var digest = [UInt8](repeating: 0, count:Int(CC_SHA1_DIGEST_LENGTH))
        data.withUnsafeBytes {
            _ = CC_SHA1($0.baseAddress, CC_LONG(data.count), &digest)
        }
        var digestInt = 0
        for i in 0..<4 {
            digestInt = digestInt << 8
            digestInt = digestInt | Int(digest[i])
        }
        return digestInt
    }

    public static func xor(_ x: Int, _ y: Int) -> Int {
        var diff = x ^ y
        var count = 0

        while diff > 0 {
            count += diff & 1
            diff = diff >> 1
        }

        return count
    }

    /// randomID generates unique 160bit random ID string
    /// - Returns: Random 160bit ID as integer.
    public static func randomID() -> Int {
        let uuidString = UUID().uuidString

        let digest = OverlayDHTUtils.sha1(string: uuidString)
        return digest
    }
}
