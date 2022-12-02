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
    static func run(_ cmd: String) throws -> Int32 {
        let task = Process()
        task.arguments = cmd.components(separatedBy: " ")
        task.executableURL = URL(fileURLWithPath: "/usr/bin/env")
        try task.run()
        task.waitUntilExit()
        return task.terminationStatus
    }
}
