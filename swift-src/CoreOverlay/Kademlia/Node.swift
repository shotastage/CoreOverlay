//
//  Node.swift
//
//
//  Created by Shota Shimazu on 2022/06/22.
//

import Foundation

public enum NetworkType {
    case `super`
    case communicative
}

open class OverlayNode {
    var networkType: NetworkType = .super

    public init() {}
}
