//
//  DHT+Utils.swift
//  
//
//  Created by Shota Shimazu on 2023/01/07.
//

import Foundation
import CommonCrypto


class OverlayDHTUtils {
    var digestLength: Int = .init(CC_SHA1_DIGEST_LENGTH)

    public static func sha1(data: Data) -> String {
        let length = Int(CC_SHA1_DIGEST_LENGTH)
        var digest = [UInt8](repeating: 0, count: length)

        let nsData = data as NSData
        CC_SHA1(nsData.bytes, CC_LONG(data.count), &digest)
        return digest.map { String(format: "%02x", $0) }.reduce("", +)
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
    /// - Returns: Random 160bit ID as string.
    public static func randomID() -> String {
        let uuidString = UUID().uuidString

        guard let data = uuidString.data(using: .utf8) else {
            return ""
        }

        let digest = OverlayDHTUtils.sha1(data: data)
        return digest
    }
}
