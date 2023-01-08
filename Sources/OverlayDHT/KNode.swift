//
//  KNode.swift
//  
//
//  Created by Shota Shimazu on 2023/01/09.
//

import Foundation



public struct KNodeNetwork {
    let ip: String
    let port: String

    static func genDefaultPort() -> String {
        return ""
    }
}


open class KNode {
    let nodeid: String
    let network: KNodeNetwork

    init() {
        nodeid = OverlayDHTUtils.randomID()
        network = KNodeNetwork(ip: "", port: KNodeNetwork.genDefaultPort())
    }
}
