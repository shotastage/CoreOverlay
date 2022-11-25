//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/11/26.
//

import Foundation


open class COLogger {
    public static func info(_ msg: String) {
        #if DEBUG
        print(msg)
        #endif
    }
}
