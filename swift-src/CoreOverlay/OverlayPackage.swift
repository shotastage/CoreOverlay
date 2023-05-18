//
//  OverlayPackage.swift
//
//
//  Created by Shota Shimazu on 2022/12/07.
//

import Foundation

public enum OverlaySupportedLanguages {
    case swift5, go, cpp
}

public enum ExecutionMode {
    case texscript, native
}

public struct OverlayPackage {
    let objects: [URL]
    let language: OverlaySupportedLanguages
    let mode: ExecutionMode
}
