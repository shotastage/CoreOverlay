//
//  Math.swift
//  
//
//  Created by Shota Shimazu on 2022/06/07.
//

import Foundation
import CommonCrypto


class CommonCryptoAlgorithm {
    var digestLength: Int = Int(CC_SHA1_DIGEST_LENGTH)

    public static func sha1(data: Data) -> String {
        let length = Int(CC_SHA1_DIGEST_LENGTH)
        var digest = Array<UInt8>(repeating: 0, count: length)

        let nsData = data as NSData
        CC_SHA1(nsData.bytes, CC_LONG(data.count), &digest)
        return digest.map { String(format: "%02x", $0) }.reduce("", +)
    }
}

open class SwiftlyDHTMath {

    public static func randomID() -> String {
         let uuidString = UUID().uuidString

         guard let data = uuidString.data(using: .utf8) else {
             return ""
         }

         let digest = CommonCryptoAlgorithm.sha1(data: data)
         return digest
     }

    public static func hummingDistance(_ x: Int, _ y: Int) -> Int {
        var diff = x ^ y
        var count = 0

        while diff > 0 {
            count += diff & 1
            diff = diff >> 1
        }

        return count
    }
}
