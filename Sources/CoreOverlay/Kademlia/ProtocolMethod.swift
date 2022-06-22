//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/06/22.
//

import Foundation


public protocol KademliaNode {
    func ping()
    func findNode()
    func store()
    func findValue()
}
