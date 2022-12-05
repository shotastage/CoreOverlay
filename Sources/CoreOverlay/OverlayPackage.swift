//
//  OverlayPackage.swift
//  
//
//  Created by Shota Shimazu on 2022/12/07.
//

import Foundation


public enum OverlaySupportedLanguages {
    case swift5
    case go
    case cpp
}

public struct OverlayPackage {
    let objects: Array<URL>
    let language: OverlaySupportedLanguages
}
