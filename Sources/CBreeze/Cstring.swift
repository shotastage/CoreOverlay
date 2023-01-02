//
//  CString.swift
//  
//
//  Created by Shota Shimazu on 2023/01/02.
//

import Foundation


// Swift String2CString Converter
// Thanks to: https://gist.github.com/yossan/51019a1af9514831f50bb196b7180107
public func toCString(from str: String) -> UnsafeMutablePointer<Int8> {
    let count = str.utf8CString.count
    let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
    // func initialize<S>(from: S) -> (S.Iterator, UnsafeMutableBufferPointer<Element>.Index)
    _ = result.initialize(from: str.utf8CString)
    return result.baseAddress!
}
