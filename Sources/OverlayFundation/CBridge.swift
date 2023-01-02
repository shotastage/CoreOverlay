//
//  CBridge.swift
//  
//
//  Created by Shota Shimazu on 2023/01/02.
//

import Foundation


// Swift String2CString Converter
public func makeCString(from str: String) -> UnsafeMutablePointer<Int8> {
    let count = str.utf8CString.count
    let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
    // func initialize<S>(from: S) -> (S.Iterator, UnsafeMutableBufferPointer<Element>.Index)
    _ = result.initialize(from: str.utf8CString)
    return result.baseAddress!
}
