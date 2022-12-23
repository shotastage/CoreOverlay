//
//  CommandProcedure.swift
//
//
//  Created by Shota Shimazu on 2022/12/12.
//

import Foundation

protocol CommandProcedure {
    
    var args: [String] { get set }
    init()
    func prepare()
    func procedure()
    func run()
}


protocol ArgumentProcedure: CommandProcedure {
    func argumentCheck()
}

extension CommandProcedure {
    
    init(arguments: [String] = []) {
        self.init()
        self.args = args
    }

    func run() {
        prepare()
        procedure()
    }
}
