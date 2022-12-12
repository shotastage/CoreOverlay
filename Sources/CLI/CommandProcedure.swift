//
//  CommandProcedure.swift
//  
//
//  Created by Shota Shimazu on 2022/12/12.
//

#if os(macOS)
import Foundation


protocol CommandProcedure {
    func prepare()
    func procedure()
    func run()
}

#endif
