//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Foundation


struct KDHTLogger {
    public static func info(_ msg: String) {
        #if DEBUG
        print(msg)
        #endif
    }
}
