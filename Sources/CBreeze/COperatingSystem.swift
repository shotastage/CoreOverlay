//
//  COperatingSystem.swift
//
//
//  Created by Shota Shimazu on 2023/09/01.
//

#if os(Linux)
    // Import Glibc on Linux
    @_exported import Glibc
#elseif os(OSX)
    // Import Darwin on macOS
    @_exported import Darwin.C
#endif
