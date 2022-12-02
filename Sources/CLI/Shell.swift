//
//  Shell.swift
//  
//
//  Created by Shota Shimazu on 2022/12/02.
//

import Foundation

#if os(macOS) || os(iOS) || os(watchOS) || os(tvOS)
import Darwin
#else
import Glibc
#endif


class Shell {
    @discardableResult
    func shell(_ url: URL = URL(fileURLWithPath: "/usr/bin/env"), _ args: String...) throws -> Int32 {
        let task = Process()
        task.arguments = args
        task.executableURL = url
        try task.run()
        task.waitUntilExit()
        return task.terminationStatus
    }
}
