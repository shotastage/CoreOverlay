//
//  Item.swift
//  ComputeDHT
//
//  Created by Shota Shimazu on 2025/01/30.
//

import Foundation
import SwiftData

@Model
final class Item {
    var timestamp: Date
    
    init(timestamp: Date) {
        self.timestamp = timestamp
    }
}
