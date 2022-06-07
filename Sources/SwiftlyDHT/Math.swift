//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/06/07.
//

import Foundation


open class SwiftlyDHTMath {
    
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
